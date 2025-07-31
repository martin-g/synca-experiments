#[synca::synca(
  #[cfg(feature = "tokio")]
  pub mod tokio { },
  #[cfg(feature = "sync")]
  pub mod sync {
    sync!();
    replace!(
      crate::one::tokio => crate::one::sync,
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
    use crate::one::tokio::func_one;

    #[tokio::test]
    async fn it_works() {
      // let result = crate::one::tokio::func_one(2, 2).await;
      let result = func_one(2, 2).await;
      assert_eq!(result, 4);
    }
  }

}
