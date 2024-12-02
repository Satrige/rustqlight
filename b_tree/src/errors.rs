use thiserror::Error;

#[derive(Error, Debug)]
pub enum BTreeCreateError {
    #[error("The leaf node degree could not be equal to 0")]
    ZeroLeafNodeDegree,

    #[error("The common node degree should be greater than 1")]
    WrongCommonNodeDegree,
}
