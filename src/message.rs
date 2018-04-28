//! A message sent over the network for peers communication.

use block::Block;

pub enum MessageLabel {
    AskLastBlock,
}

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
}
