use block_tools::{
	auth::{optional_token, optional_validate_token},
	blocks::Context,
	display_api::component::{
		card::{CardComponent, CardHeader},
		icon::Icon,
		menu::MenuComponent,
		text::TextComponent,
		DisplayComponent,
	},
	models::Block,
};

pub fn embed_display(block: &Block, context: &Context) -> Box<dyn DisplayComponent> {
	let user_id = optional_validate_token(optional_token(context)).unwrap();
	let data: Option<String> = block.block_data.clone();

	let card_content = TextComponent::new(&data.unwrap_or_else(|| "".into()));

	let mut header = CardHeader::new("Data").icon(Icon::Box).id(block.id);
	if let Some(user_id) = user_id {
		header.menu = Some(MenuComponent::load_from_block(block, user_id));
	}

	let component = CardComponent {
		content: Box::new(card_content),
		color: block.color.clone(),
		header,
	};
	Box::new(component)
}
