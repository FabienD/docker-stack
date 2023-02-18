#[cfg(test)]
mod tests {
    use crate::utils::docker::{Container, Docker};

    #[test]
    fn it_initializes_with_bin_path() {
        let docker: Docker = Container::init("../../tests/fake_docker".to_string());
        assert!(
            docker
                == Docker {
                    bin_path: "../../tests/fake_docker".to_string()
                }
        );
    }
}
