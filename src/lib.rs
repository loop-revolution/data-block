use async_trait::async_trait;
use block_tools::{Error, blocks::{BlockType, Context}, display_api::{
		component::{
			card::{CardComponent, CardHeader, CardIcon},
			input::InputComponent,
			text::{TextComponent, TextPreset},
			DisplayComponent,
		},
		CreationObject, DisplayMeta, DisplayObject, PageMeta,
	}, models::{Block, MinNewBlock}};

pub struct DataBlock {}

#[async_trait]
impl BlockType for DataBlock {
	fn name(&self) -> &str {
		"data"
	}

	async fn page_display(block: &Block, _context: &Context) -> Result<DisplayObject, Error> {
		let data = block.block_data.clone();
		let data_string = &data.unwrap_or("".into());
		let component = TextComponent::new(data_string);

		let mut page = PageMeta::new().title("Data");

		page = match data_string.as_str() {
			"" => page,
			_ => page.header(data_string),
		};

		let meta = DisplayMeta::new().page(page);
		Ok(DisplayObject::new(Box::new(component)).meta(meta))
	}

	async fn embed_display(
		block: &Block,
		_context: &Context,
	) -> Result<Box<dyn DisplayComponent>, Error> {
		let data: Option<String> = block.clone().block_data.clone();

		let card_content = TextComponent::new(&data.unwrap_or("".into()));
		let component = CardComponent {
			content: Box::new(card_content),
			color: None,
			header: CardHeader {
				title: "Data".into(),
				icon: Some(CardIcon::Box),
				block_id: Some(block.id.to_string()),
			},
		};
		Ok(Box::new(component))
	}

	async fn create_display(_context: &Context, _user_id: i32) -> Result<CreationObject, Error> {
		let header = TextComponent::new("New Data Block").preset(TextPreset::Heading);
		let main = InputComponent::new().label("Data").name("DATA");
		let object = CreationObject {
			header_component: Box::new(header),
			main_component: Box::new(main),
			input_template: "$[DATA]$".into(),
		};
		Ok(object)
	}

	async fn create(input: String, context: &Context, user_id: i32) -> Result<Block, Error> {
		let conn = &context.pool.get()?;

		let block = MinNewBlock {
			block_type: "data",
			owner_id: user_id,
		}
		.into()
		.data(&input);

		Ok(block.insert(conn)?)
	}
}
