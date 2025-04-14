use datafusion::functions_aggregate::sum::sum;
use datafusion::prelude::*;

#[tokio::main]
async fn main() -> datafusion::error::Result<()>{
    test_sql2().await?;
    // test_case45_via_sql().await?;
    // test_case45_via_dataframe().await?;
    // test_case45_via_dataframe2().await?;
    Ok(())
}


#[tokio::test]
async fn test_sql1() -> datafusion::error::Result<()> {
    let ctx = SessionContext::new();

    ctx.register_csv("users", "data/users.csv", CsvReadOptions::new()).await?;
    let df = ctx.sql("SELECT * FROM users where age >= 30").await?;

    df.show().await?;

    Ok(())
}

#[tokio::test]
async fn test_dataframe1() -> datafusion::error::Result<()> {
    use datafusion::functions_aggregate::count::count;
    let ctx = SessionContext::new();

    let table = ctx.read_csv("data/users.csv", CsvReadOptions::new()).await?;

    let result = table.filter( col("age").gt_eq(lit(20)) )?
        .aggregate(vec![col("sex")], 
                   vec![count(col("sex")), sum(col("age"))])?
        .sort_by(vec![col("sex")])?;
    result.clone().explain(true, true)?.show().await?;
    result.show().await?;

    Ok(())
}

async fn prepare_dataset(ctx: &SessionContext) -> datafusion::error::Result<()> {
    
    let root = format!("{}/workspaces/wangzaixiang/mpp_test/datafusion", std::env::var("HOME").unwrap());
    
    ctx.register_parquet("sale_items", format!("{root}/sale_items.parquet"), ParquetReadOptions::default()).await?;
    ctx.register_parquet("sale_orders", format!("{root}/sale_orders.parquet"), ParquetReadOptions::default()).await?;
    ctx.register_parquet("customers", format!("{root}/customers.parquet"), ParquetReadOptions::default()).await?;
    ctx.register_parquet("customer_tags", format!("{root}/customer_tags.parquet"), ParquetReadOptions::default()).await?;
    ctx.register_parquet("products", format!("{root}/products.parquet"), ParquetReadOptions::default()).await?;
    ctx.register_parquet("tags", format!("{root}/tags.parquet"), ParquetReadOptions::default()).await?;
    ctx.register_parquet("purchase_items", format!("{root}/purchase_items.parquet"), ParquetReadOptions::default()).await?;
    ctx.register_parquet("purchase_orders", format!("{root}/purchase_orders.parquet"), ParquetReadOptions::default()).await?;
    ctx.register_parquet("suppliers", format!("{root}/suppliers.parquet"), ParquetReadOptions::default()).await?;

    Ok(())
}

async fn test_case45_via_sql() -> datafusion::error::Result<()> {
    let ctx = SessionContext::new();
    prepare_dataset(&ctx).await?;

    let time0 = std::time::Instant::now();
    let df = ctx.sql("select wt.tag_name, sum(wt.amount) from (
	select si.sale_item_id as sale_item_id, 
		si.sale_order_id as sale_order_id,
		si.product_id as product_id,
		si.quantity as quantity,
		si.price as price, 
		si.amount as amount,
		s.order_date as order_date,
		s.shop_id as shop_id,
		s.freight as freight,
		c.customer_id as customer_id,
		c.name as customer_name,
		t.tag_name as tag_name
	from sale_items si
	 left join sale_orders s on si.sale_order_id = s.sale_order_id
	 left join customers c on s.customer_id = c.customer_id
	 left join customer_tags ct on ct.customer_id = c.customer_id
     left join tags t on t.tag_id = ct.tag_id 
) as wt where wt.tag_name = 'tag1' group by wt.tag_name").await?;

    let tm0 = std::time::Instant::now();
    df.clone().explain(false, true)?.show().await?;
    let tm1 = std::time::Instant::now();
    println!("explain time: {:?}", tm1.duration_since(tm0));
    // df.show().await?;
    let time1 = std::time::Instant::now();
    println!("test_case45_via_sql time: {:?}", time1.duration_since(time0));

    Ok(())
}

async fn test_sql2() -> datafusion::error::Result<()> {
    let ctx = SessionContext::new();
    prepare_dataset(&ctx).await?;

    let time0 = std::time::Instant::now();
    let df = ctx.sql("select s.order_date, sum(si.amount) from sale_items si
     left join sale_orders s on si.sale_order_id = s.sale_order_id group by s.order_date").await?;

    df.clone().explain(false, true)?.show().await?;
    let time1 = std::time::Instant::now();
    // let time1 = time0.clone();

    df.show().await?;
    let time2 = std::time::Instant::now();
    println!("test_sql2 explain {:?}, execute: {:?}", time1.duration_since(time0), time2.duration_since(time1));

    Ok(())
}

async fn test_case45_via_dataframe() -> datafusion::error::Result<()> {
    let ctx = SessionContext::new();
    
    let root = format!("{}/workspaces/wangzaixiang/mpp_test/datafusion", std::env::var("HOME").unwrap());
    
    // 注册所有表
    let sale_items = ctx.read_parquet(format!("{root}/sale_items.parquet"), ParquetReadOptions::default()).await?.alias("si")?;
    let sale_orders = ctx.read_parquet(format!("{root}/sale_orders.parquet"), ParquetReadOptions::default()).await?.alias("so")?;
    let customers = ctx.read_parquet(format!("{root}/customers.parquet"), ParquetReadOptions::default()).await?.alias("c")?;
    let customer_tags = ctx.read_parquet(format!("{root}/customer_tags.parquet"), ParquetReadOptions::default()).await?.alias("ct")?;
    let tags = ctx.read_parquet(format!("{root}/tags.parquet"), ParquetReadOptions::default()).await?.alias("t")?;
    
    // 构建查询
    let time0 = std::time::Instant::now();
    
    // 执行连接操作
    let result = sale_items
        .join(
            sale_orders,
            JoinType::Left,
            &["sale_order_id"],
            &["sale_order_id"],
            None,
        )?
        .join(
            customers,
            JoinType::Left,
            &["customer_id"],
            &["customer_id"],
            None,
        )?
        .join(
            customer_tags,
            JoinType::Left,
            &["c.customer_id"],
            &["customer_id"],
            None,
        )?
        .join(
            tags,
            JoinType::Left,
            &["tag_id"],
            &["tag_id"],
            None,
        )?
        // 选择需要的列
        .select(vec![
            col("sale_item_id"),
            col("so.sale_order_id"),
            col("product_id"),
            col("quantity"),
            col("price"),
            col("amount"),
            col("order_date"),
            col("shop_id"),
            col("freight"),
            col("c.customer_id"),
            col("c.name"),
            col("tag_name"),
        ])?
        // 过滤 tag_name = 'tagx'
        .filter(col("tag_name").eq(lit("tagx")))?
        // 分组和聚合
        .aggregate(
            vec![col("tag_name")],
            vec![sum(col("amount"))],
        )?;

    // result.clone().explain(true, false)?.show().await?;
    
    // 显示结果
    result.show().await?;
    
    let time1 = std::time::Instant::now();
    println!("test_case45_via_dataframe time: {:?}", time1.duration_since(time0));

    Ok(())
}

// optimize join order
async fn test_case45_via_dataframe2() -> datafusion::error::Result<()> {
    let ctx = SessionContext::new();

    let root = format!("{}/workspaces/wangzaixiang/mpp_test/datafusion", std::env::var("HOME").unwrap());

    // 注册所有表
    let sale_items = ctx.read_parquet(format!("{root}/sale_items.parquet"), ParquetReadOptions::default()).await?.alias("si")?;
    let sale_orders = ctx.read_parquet(format!("{root}/sale_orders.parquet"), ParquetReadOptions::default()).await?.alias("so")?;
    let customers = ctx.read_parquet(format!("{root}/customers.parquet"), ParquetReadOptions::default()).await?.alias("c")?;
    let customer_tags = ctx.read_parquet(format!("{root}/customer_tags.parquet"), ParquetReadOptions::default()).await?.alias("ct")?;
    let tags = ctx.read_parquet(format!("{root}/tags.parquet"), ParquetReadOptions::default()).await?.alias("t")?;

    // 构建查询
    let time0 = std::time::Instant::now();

    let ct_t = customer_tags.join(
        tags,
        JoinType::Left,
        &["tag_id"],
        &["tag_id"],
        Some( col("tag_name").eq(lit("tagx")) ),
    )?;

    let c_ct_t = customers.join(
        ct_t,
        JoinType::Inner,
        &["customer_id"],
        &["customer_id"],
        None,
    )?;

    let so_c_ct_t = sale_orders.join(
        c_ct_t,
        JoinType::Inner,
        &["customer_id"],
        &["c.customer_id"],
        None,
    )?;

    let si_so_c_ct_t = sale_items.join(
        so_c_ct_t,
        JoinType::Inner,
        &["sale_order_id"],
        &["sale_order_id"],
        None,
    )?;

    // 执行连接操作
    let result = si_so_c_ct_t
        .select(vec![
            col("sale_item_id"),
            col("so.sale_order_id"),
            col("product_id"),
            col("quantity"),
            col("price"),
            col("amount"),
            col("order_date"),
            col("shop_id"),
            col("freight"),
            col("c.customer_id"),
            col("c.name"),
            col("tag_name"),
        ])?
        // 过滤 tag_name = 'tagx'
        .filter(col("tag_name").eq(lit("tagx")))?
        // 分组和聚合
        .aggregate(
            vec![col("tag_name")],
            vec![sum(col("amount"))],
        )?;

    // result.clone().explain(false, true)?.show().await?;

    // 显示结果
    result.show().await?;

    let time1 = std::time::Instant::now();
    println!("test_case45_via_dataframe2 time: {:?}", time1.duration_since(time0));

    Ok(())
}