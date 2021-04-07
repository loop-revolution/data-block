use block_tools::{Error, blocks::{BlockType, Context, TypeInfo}, display_api::{CreationObject, DisplayObject, component::{DisplayComponent, atomic::icon::Icon}}, models::Block};

pub mod display;
mod methods;

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

	fn page_display(block: &Block, context: &Context) -> Result<DisplayObject, Error> {
		Self::handle_page_display(block, context)
	}

	fn embed_display(block: &Block, context: &Context) -> DisplayComponent {
		Self::handle_embed_display(block, context)
	}

	fn create_display(_context: &Context, _user_id: i32) -> Result<CreationObject, Error> {
		Self::handle_create_display()
	}

	fn create(input: String, context: &Context, user_id: i32) -> Result<Block, Error> {
		Self::handle_create(input, context, user_id)
	}

	fn method_delegate(
		context: &Context,
		name: String,
		block_id: i64,
		args: String,
	) -> Result<Block, Error> {
		Self::handle_method_delegate(context, name, block_id, args)
	}

	fn block_name(_block: &Block, _context: &Context) -> Result<String, Error> {
		Ok("Data Block".into())
	}
}
