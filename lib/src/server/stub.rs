use std::{convert::{TryFrom, TryInto}, fs::OpenOptions, path::PathBuf};

use wiremock::Mock;

use super::super::model::stub::StubDto;

pub struct StubrMock(pub Mock);

impl TryFrom<&PathBuf> for StubrMock {
    type Error = anyhow::Error;

    fn try_from(maybe_stub: &PathBuf) -> anyhow::Result<Self> {
        let file = OpenOptions::new().read(true).open(&maybe_stub)?;
        let stub: StubDto = serde_json::from_reader(file)?;
        Ok(Self { 0: stub.try_into()? })
    }
}
