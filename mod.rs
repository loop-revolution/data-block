use block_tools::{
	auth::{
		permissions::{has_perm_level, PermLevel},
		require_token, validate_token,
	},
	blocks::{BlockType, Context, TypeInfo},
	display_api::{
		component::{
			card::{CardComponent, CardHeader, Icon},
			input::InputComponent,
			text::{TextComponent, TextPreset},
			DisplayComponent,
		},
		CreationObject, DisplayMeta, DisplayObject, MethodObject, PageMeta,
	},
	models::{Block, MinNewBlock},
	BlockError, Error,
};

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

	fn page_display(block: &Block, _context: &Context) -> Result<DisplayObject, Error> {
		let data = block.block_data.clone();
		let data_string = &data.unwrap_or_else(|| "".into());
		let component = edit_data_component(block.id.to_string())
			.initial_value(data_string)
			.label("Data");

		let mut page = PageMeta::new().title("Data");

		page = match data_string.as_str() {
			"" => page,
			_ => page.header(data_string),
		};

		let meta = DisplayMeta::default().page(page);
		Ok(DisplayObject::new(Box::new(component)).meta(meta))
	}

	fn embed_display(block: &Block, _context: &Context) -> Box<dyn DisplayComponent> {
		let data: Option<String> = block.block_data.clone();

		let card_content = TextComponent::new(&data.unwrap_or_else(|| "".into()));
		let component = CardComponent {
			content: Box::new(card_content),
			color: None,
			header: CardHeader {
				title: "Data".into(),
				icon: Some(Icon::Box),
				block_id: Some(block.id.to_string()),
				menu: None,
			},
		};
		Box::new(component)
	}

	fn create_display(_context: &Context, _user_id: i32) -> Result<CreationObject, Error> {
		let header = TextComponent::new("New Data Block").preset(TextPreset::Heading);
		let main = InputComponent::new().label("Data").name("DATA");
		let object = CreationObject {
			header_component: Box::new(header),
			main_component: Box::new(main),
			input_template: "$[DATA]$".into(),
		};
		Ok(object)
	}

	fn create(input: String, context: &Context, user_id: i32) -> Result<Block, Error> {
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

	fn method_delegate(
		context: &Context,
		name: String,
		block_id: i64,
		args: String,
	) -> Result<Block, Error> {
		match name.as_str() {
			"edit" => edit(context, block_id, args),
			_ => Err(BlockError::MethodExist(name, DataBlock::name()).into()),
		}
	}

	fn block_name(_block: &Block, _context: &Context) -> Result<String, Error> {
		Ok("Data Block".into())
	}
}

fn edit(context: &Context, block_id: i64, args: String) -> Result<Block, Error> {
	let conn = &context.pool.get()?;
	let user_id = validate_token(&require_token(context)?)?;
	let access_err: Error =
		BlockError::TypeGenericError(format!("Cannot edit data block {}", block_id)).into();
	let block = Block::by_id(block_id, conn)?;
	let block = match block {
		Some(b) => b,
		None => return Err(access_err),
	};
	if !has_perm_level(user_id, &block, PermLevel::Edit) {
		return Err(access_err);
	}
	let mut input = args;
	input.remove(0);
	input.pop();
	let block = block.update_data(&input, conn)?;
	Ok(block)
}

pub fn edit_data_component(block_id: String) -> InputComponent {
	let method = MethodObject {
		block_type: DataBlock::name(),
		block_id,
		method_name: "edit".into(),
		arg_template: "$[DATA]$".into(),
	};
	InputComponent::new()
		.name("DATA")
		.with_confirm(method.into())
}
