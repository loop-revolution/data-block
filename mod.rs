use block_tools::{
	blocks::{BlockType, Context, TypeInfo},
	display_api::{
		component::{icon::Icon, DisplayComponent},
		CreationObject, DisplayObject,
	},
	models::Block,
	LoopError,
};

use self::display::{create::create_display, embed::embed_display, page::page_display};
mod display;
mod methods;
pub use display::masked_data_edit::masked_data_edit;

pub const BLOCK_NAME: &str = "data";

pub struct DataBlock {}

impl BlockType for DataBlock {
	fn name() -> String {
		BLOCK_NAME.to_string()
	}

	fn info() -> TypeInfo {
		TypeInfo {
			name: Self::name(),
			icon: Icon::Box,
			desc: "The simplest block that represents raw rext and nothing more".to_string(),
		}
	}

	fn page_display(block: &Block, context: &Context) -> Result<DisplayObject, LoopError> {
		page_display(block, context)
	}

	fn embed_display(block: &Block, context: &Context) -> Box<dyn DisplayComponent> {
		embed_display(block, context)
	}

	fn create_display(_context: &Context, _user_id: i32) -> Result<CreationObject, LoopError> {
		create_display()
	}

	fn create(input: String, context: &Context, user_id: i32) -> Result<Block, LoopError> {
		methods::create::create(input, context, user_id)
	}

	fn method_delegate(
		context: &Context,
		name: String,
		block_id: i64,
		args: String,
	) -> Result<Block, LoopError> {
		methods::method_delegate(context, name, block_id, args)
	}

	fn block_name(_block: &Block, _context: &Context) -> Result<String, LoopError> {
		Ok("Data Block".into())
	}
}
