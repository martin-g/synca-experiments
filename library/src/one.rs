#[synca::synca(
  #[cfg(feature = "tokio")]
  pub mod tokio { },
  #[cfg(feature = "sync")]
  pub mod sync {
    sync!();
    replace!(
      #[tokio::test] => #[test]
    );
  }
)]
mod one {

  pub async fn func_one(left: u64, right: u64) -> u64 {
    left + right
  }


  #[cfg(test)]
  mod tests {
    use crate::one;

    #[test]
    #[cfg(feature = "sync")]
    fn sync_works() {
      let result = one::sync::func_one(2, 2);
      assert_eq!(result, 4);
    }

    #[tokio::test]
    #[cfg(feature = "tokio")]
    async fn async_works() {
      let result = one::tokio::func_one(2, 2).await;
      // assert_eq!(result, 4);
      // dbg!(result);
      println!("Result: {}", result);
    }
  }

}
