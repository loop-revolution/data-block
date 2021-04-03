use crate::blocks::data_block::DataBlock;
use block_tools::blocks::BlockType;
use block_tools::{
	blocks::Context,
	models::{Block, MinNewBlock},
	Error,
};

impl DataBlock {
	pub fn handle_create(input: String, context: &Context, user_id: i32) -> Result<Block, Error> {
		let conn = &context.pool.get()?;
		let mut input = input;
		input.remove(0);
		input.pop();

		let block = MinNewBlock {
			block_type: &DataBlock::name(),
			owner_id: user_id,
		}
		.into()
		.data(&input);

		block.insert(conn)
	}
}
