use super::*;

pub trait Entity: PartialEq {
    fn id(&self) -> &Id<Self>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    struct TestEntity {
        id: Id<TestEntity>,
    }

    impl Entity for TestEntity {
        fn id(&self) -> &Id<Self> {
            &self.id
        }
    }
    impl PartialEq for TestEntity {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
        }
    }

    #[test_case("hoge" => Id::<TestEntity>::new("hoge".to_string()))]
    #[test_case("hoge2" => Id::<TestEntity>::new("hoge2".to_string()))]
    fn works_id(v1: &str) -> Id<TestEntity> {
        let e = TestEntity {
            id: Id::new(v1.to_string()),
        };
        e.id().clone()
    }
    #[test_case("hoge","hoge" => true)]
    #[test_case("hoge","foo" => false)]
    fn works_eq(v1: &str, v2: &str) -> bool {
        let e1 = TestEntity {
            id: Id::new(v1.to_string()),
        };
        let e2 = TestEntity {
            id: Id::new(v2.to_string()),
        };
        e1 == e2
    }
}
