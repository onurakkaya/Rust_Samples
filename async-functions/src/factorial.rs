use async_recursion::async_recursion;

#[async_recursion]
pub async fn calculate(mut _value: u128) -> u128{
    if _value >= 1 {
        let valx = _value - 1;
        return _value * calculate(valx).await;
    }
    return 1;
}