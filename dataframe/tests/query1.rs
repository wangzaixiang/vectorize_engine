use std::error::Error;
use dataframe::DataFrame;

/// table: users(id, name, birthday, email)
/// query: select name, email from users where birthday > '1990-01-01'
///
/// let df1 = ... // df1: DataFrame[ Series("id", U64), Series("name", String), Series("birthday", Date), Series("email", String) ]
/// let df2 = df1.project([1, 2, 3])  // df2: DataFrame[ Series("name", String), Series("birthday", Date), Series("email", String) ]
/// let df3 = df2.filter(|row| row[1]: date > date'1990-01-01')  // df3: DataFrame[ Series("name", String), Series("email", String) ]
/// let df4 = df3.project([0, 1])  // df4: DataFrame[ Series("name", String), Series("email", String) ]
/// or
/// let df4 = df2.filter_and_project(|row| row[1]: date > date'1990-01-01', [0, 2])  // df4: DataFrame[ Series("name", String), Series("email", String) ]

#[test]
fn query1() -> Result<(), &dyn Error> {

    // import the from_str function from datatype.rs
    use dataframe::datatype::Date;

    let id = vec![1, 2, 3, 4, 5];   // vector!
    let name = vec!["Alice", "Bob", "Charlie", "David", "Eve"];
    let birthday = vec![ Date::from_str("1980-01-01")?, Date::from_str("1990-01-01")?,
                         Date::from_str("2000-01-01")?, Date::from_str("2010-01-01")?, Date::from_str("2020-01-01")? ];


    let id = vector!(id);  //
    let name = vector!(name);
    let birthday = vector!(birthday);

    let df = dataframe!("id", id, "name", name, "birthday", birthday);

    let pipeline = Pipeline::new();
    pipeline.set_source( DataFrameSource::new(df) );

    let date1 = Date::from_str("1990-01-01")?;

    let filter = |df: &dyn DataFrame| {   // TODO, optimize into vectorized code
        let birthday = df.series::<Date>(2);
        birthday.filter(|date| date > date1)    //
    };

    pipeline.add(filter);

    let sink = DataFrameSink::new();
    pipeline.set_sink( sink );

    pipeline.execute();

    println!("Result: {:?}", sink.get_data_frame());

    Ok(())

}


/// table: order_item(order_item_id, order_date, product_id, quantity, amount)
/// table: product(product_id, product_name, category_id)
/// query: select product_name, sum(quantity), sum(amount) from order_item join product on order_item.product_id = product.product_id group by product_name
///
/// let pipeline1 = pipeline();
/// let pipeline2 = pipeline();
///
/// let df1 = scan_table("order_item", ["product_id", "quantity", "amount"]
/// -- df1: DataFrame[ Series("product_id", U64), Series("quantity", U64), Series("amount", U64) ]
///
/// let df2 = scan_table("product", ["product_id", "product_name"]
/// -- df2: DataFrame[ Series("product_id", U64), Series("product_name", String) ]
///
/// let ht_build1 = build_hash_table(input = df2, key = "product_id")
///     -- key: [ Series("product_id", U64) ]
///     -- value: [ Series("product_name", String) ]
/// let ht_lookup1 = lookup_hash_table(ht_build, input = df1, key = "product_id", output = [["product_name"], ["quantity", "amount"]])
///     -- DataFrame[ Series("product_name", String), Series("quantity", U64), Series("amount", U64) ]
///
/// let hash_aggr1 = hash_aggregate(input = ht_lookup, key = ["product_name"],
///                 aggr = [sum("quantity"), sum("amount")])
///     -- DataFrame[ Series("product_name", String), Series("sum(quantity)", U64), Series("sum(amount)", U64) ]
///
/// let pipeline1 = pipeline( source = df1, sink = ht_build );
/// let pipeline2 = pipeline( source = df2, operatoes = [ht_lookup, hash_aggr1], sink = identity_sink(), parent = pipeline1 );
/// let topology = [pipeline1, pipeline2];
///
fn query2() {

}

/// table: order_item(order_item_id, order_date, user_id, product_id, quantity, amount)
/// table: users(user_id, name, sex, province, city)
/// query: select province, city, count(order_item_id), sum(quantity), sum(amount)
///         from order_item join users on order_item.user_id = users.user_id
///         where order_date >= '2025-01-1' and sex = 'F'
///         group by province, city
///
/// let df1 = scan_table("order_item", ["user_id", "date", "quantity", "amount"], ["order_date", |date| date >= '2025-01-01'])
/// let df2 = scan_table("users", ["user_id", "sex", "province", "city"], ["sex", |sex| sex == 'F'] )
/// let filter1 = filter(input = df1, predicate = |row| row[1]: date >= date'2025-01-01')
/// let filter2 = filter(input = df2, predicate = |row| row[2]: string == 'F')
/// let hash_build1 = build_hash_table(input = filter2, key = "user_id", value = ["province", "city"])
/// let hash_lookup1 = lookup_hash_table(right = ht_build1, input = df1, key = "user_id", output = [["province", "city"], ["quantity", "amount"]])