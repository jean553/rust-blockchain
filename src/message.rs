//! A message sent over the network for peers communication.

use block::Block;

#[derive(Serialize, Deserialize, PartialEq)]
pub enum MessageLabel {
    AskForAllBlocks,
    SendBlock,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    blocks: Vec<Block>,
    label: MessageLabel,
}

impl Message {

    /// Message constructor.
    ///
    /// Args:
    ///
    /// `blocks` - the blocks chain to include into the message
    /// `label` - the label of the message (type)
    ///
    /// Returns:
    ///
    /// the new message
    pub fn new(blocks: Vec<Block>, label: MessageLabel) -> Message {
        Message {
            blocks: blocks,
            label: label,
        }
    }

    /// Getter of the label
    ///
    /// Return:
    ///
    /// returns the message label
    pub fn get_label(&self) -> &MessageLabel {
        &self.label
    }

    /// Getter of the blocks array
    ///
    /// Return:
    ///
    /// the blocks array
    pub fn get_blocks(&self) -> &Vec<Block> {
        &self.blocks
    }

    /// Setter of the blocks array
    ///
    /// Args:
    ///
    /// `blocks` - the blocks array to set
    pub fn set_blocks(&mut self, blocks: Vec<Block>) {
        self.blocks = blocks;
    }
}
