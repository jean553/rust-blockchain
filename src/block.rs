//! A chain block.

use bincode::serialize;
use sha1::Sha1;

use hash_content::HashContent;

#[derive(Serialize, Deserialize, Clone)]
pub struct Block {
    content: HashContent,
    previous: String,
    current: String,
}

impl Block {

    /// One block constructor. Creates the block from the given data and previous digest. Calculates its own hash digest.
    ///
    /// Args:
    ///
    /// `data` - the data of the block
    /// `previous` - the digest of the previous block (empty if genesis)
    ///
    /// Returns:
    ///
    /// new block
    pub fn new(
        data: i32,
        previous: String,
    ) -> Block {

        let content = HashContent::new(data);
        let bytes = serialize(&content).unwrap();
        let digest = Sha1::from(bytes).hexdigest();

        Block {
            content: content,
            previous: previous,
            current: digest,
        }
    }

    /// Getter of the current block hash digest.
    ///
    /// Returns:
    ///
    /// current block digest as string
    pub fn get_current(&self) -> &str {
        &self.current
    }

    /// Getter of the hashed content.
    ///
    /// Returns:
    ///
    /// block hashed content
    pub fn get_content(&self) -> &HashContent {
        &self.content
    }
}

