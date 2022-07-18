use solscan_api::structs::block_result::BlockResult;

pub fn calculate_tps(block_results: Vec<BlockResult>) -> i64 {
    let mut transaction_sum = 0;

    let time_first_block = block_results.first().unwrap().result.as_ref().unwrap().block_time.unwrap_or_default();
    let time_last_block = block_results.last().unwrap().result.as_ref().unwrap().block_time.unwrap_or_default();

    for block_result in block_results {
        transaction_sum += block_result.result.unwrap().transaction_count.unwrap_or(0)
    };

    /* println!("time_first_block: {:?}", time_first_block);
     println!("time_last_block: {:?}", time_last_block);
     println!("transaction_sum: {:?}", transaction_sum);*/

    if time_first_block != 0 && time_last_block != 0 {
        transaction_sum / (time_first_block - time_last_block)
    } else { 0 }
}