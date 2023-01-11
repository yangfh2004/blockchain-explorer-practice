#[cfg(test)]
mod tests {
    use crate::blocks;
    use crate::{Service, ServiceImpl, DB_NAME};

    fn assert_balances<S: Service>(
        service: &S,
        expected_balance_alice: anyhow::Result<S::Balance>,
        expected_balance_bob: anyhow::Result<S::Balance>,
    ) {
        if let Ok(balance) = expected_balance_alice {
            assert_eq!(service.get_balance(&blocks::ALICE).unwrap(), balance);
        } else {
            panic!("Alice's balance is not available!");
        }

        if let Ok(balance) = expected_balance_bob {
            assert_eq!(service.get_balance(&blocks::BOB).unwrap(), balance);
        } else {
            panic!("Bob's balance is not available!");
        }
    }

    #[test]
    fn service_restart() {
        let mut service = ServiceImpl::new();
        service.ingest_block(&blocks::BLOCK_A).unwrap();
        service.ingest_block(&blocks::BLOCK_B).unwrap();
        assert_balances(&service, anyhow::Ok(10), anyhow::Ok(0));
        // get temporary dir path.
        let dir = service.path.unwrap();
        // restart service.
        let service = ServiceImpl::from_db(dir.as_str(), DB_NAME);
        assert_balances(&service, anyhow::Ok(10), anyhow::Ok(0));
    }

    #[test]
    fn test_1() {
        // single block
        let mut service = ServiceImpl::new();
        assert_balances(&service, anyhow::Ok(0), anyhow::Ok(0));
        service.ingest_block(&blocks::BLOCK_A).unwrap();
        assert_balances(&service, anyhow::Ok(5), anyhow::Ok(5));
    }

    #[test]
    fn test_2() {
        // single chain of blocks
        let mut service = ServiceImpl::new();
        assert_balances(&service, anyhow::Ok(0), anyhow::Ok(0));
        service.ingest_block(&blocks::BLOCK_A).unwrap();
        assert_balances(&service, anyhow::Ok(5), anyhow::Ok(5));
        service.ingest_block(&blocks::BLOCK_B).unwrap();
        assert_balances(&service, anyhow::Ok(10), anyhow::Ok(0));
    }

    #[test]
    fn test_3() {
        // multiple forks
        let mut service = ServiceImpl::new();
        assert_balances(&service, anyhow::Ok(0), anyhow::Ok(0));
        service.ingest_block(&blocks::BLOCK_A).unwrap();
        assert_balances(&service, anyhow::Ok(5), anyhow::Ok(5));
        service.ingest_block(&blocks::BLOCK_B).unwrap();
        assert_balances(&service, anyhow::Ok(10), anyhow::Ok(0));
        service.ingest_block(&blocks::BLOCK_C).unwrap();
        assert_balances(&service, anyhow::Ok(8), anyhow::Ok(2));
    }

    #[test]
    fn test_4() {
        // multiple forks, different order
        let mut service = ServiceImpl::new();
        assert_balances(&service, anyhow::Ok(0), anyhow::Ok(0));
        service.ingest_block(&blocks::BLOCK_A).unwrap();
        assert_balances(&service, anyhow::Ok(5), anyhow::Ok(5));
        service.ingest_block(&blocks::BLOCK_C).unwrap();
        assert_balances(&service, anyhow::Ok(8), anyhow::Ok(2));
        service.ingest_block(&blocks::BLOCK_B).unwrap();
        assert_balances(&service, anyhow::Ok(10), anyhow::Ok(0));
    }

    #[test]
    fn test_5() {
        // multiple forks where the longest chain changes mid stream
        let mut service = ServiceImpl::new();
        assert_balances(&service, anyhow::Ok(0), anyhow::Ok(0));
        service.ingest_block(&blocks::BLOCK_A).unwrap();
        assert_balances(&service, anyhow::Ok(5), anyhow::Ok(5));
        service.ingest_block(&blocks::BLOCK_B).unwrap();
        assert_balances(&service, anyhow::Ok(10), anyhow::Ok(0));
        service.ingest_block(&blocks::BLOCK_C).unwrap();
        assert_balances(&service, anyhow::Ok(8), anyhow::Ok(2));
        service.ingest_block(&blocks::BLOCK_D).unwrap();
        assert_balances(&service, anyhow::Ok(6), anyhow::Ok(4));
    }

    #[test]
    fn test_6() {
        // multiple forks with the longest fork arriving out of order
        let mut service = ServiceImpl::new();
        assert_balances(&service, anyhow::Ok(0), anyhow::Ok(0));
        service.ingest_block(&blocks::BLOCK_A).unwrap();
        assert_balances(&service, anyhow::Ok(5), anyhow::Ok(5));
        service.ingest_block(&blocks::BLOCK_B).unwrap();
        assert_balances(&service, anyhow::Ok(10), anyhow::Ok(0));
        service.ingest_block(&blocks::BLOCK_D).unwrap();
        assert_balances(&service, anyhow::Ok(10), anyhow::Ok(0));
        service.ingest_block(&blocks::BLOCK_C).unwrap();
        assert_balances(&service, anyhow::Ok(8), anyhow::Ok(2));
    }

    // multiple forks where the genesis block is the last to arrive
    #[test]
    fn test_7() {
        let mut service = ServiceImpl::new();
        assert_balances(&service, anyhow::Ok(0), anyhow::Ok(0));
        service.ingest_block(&blocks::BLOCK_D).unwrap();
        assert_balances(&service, anyhow::Ok(0), anyhow::Ok(0));
        service.ingest_block(&blocks::BLOCK_B).unwrap();
        assert_balances(&service, anyhow::Ok(0), anyhow::Ok(0));
        service.ingest_block(&blocks::BLOCK_C).unwrap();
        assert_balances(&service, anyhow::Ok(0), anyhow::Ok(0));
        service.ingest_block(&blocks::BLOCK_A).unwrap();
        assert_balances(&service, anyhow::Ok(5), anyhow::Ok(5));
    }
}
