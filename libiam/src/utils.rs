use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}
