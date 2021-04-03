use block_tools::{
	auth::{
		optional_token, optional_validate_token,
		permissions::{has_perm_level, PermLevel},
	},
	blocks::Context,
	display_api::{
		component::{input::InputSize, menu::MenuComponent, text::TextComponent, DisplayComponent},
		DisplayMeta, DisplayObject, PageMeta,
	},
	models::Block,
	Error,
};

use crate::blocks::data_block::DataBlock;

impl DataBlock {
	pub fn handle_page_display(block: &Block, context: &Context) -> Result<DisplayObject, Error> {
		let user_id = optional_validate_token(optional_token(context))?;

		// Make access to data details easier
		let data = block.block_data.clone();
		let data_string = &data.unwrap_or_else(|| "".into());

		// Display API to render
		let text_component = TextComponent::new(data_string);
		let mut component: Box<dyn DisplayComponent> = box text_component.clone();
		let mut page = PageMeta::new()
			.title("Data")
			.header(&format!("Data Block #{}", block.id));

		if let Some(user_id) = user_id {
			// Add a menu to the page
			page.menu = Some(MenuComponent::load_from_block(block, user_id));
			// If the user can edit it the data, make it possible to edit
			if has_perm_level(user_id, block, PermLevel::Edit) {
				component = box Self::masked_editable_data(
					block.id.to_string(),
					block.block_data.clone(),
					false,
				)
				.initial_value(data_string)
				.label("Data")
				.size(InputSize::MultiLine)
			}
		}

		let meta = DisplayMeta::default().page(page);
		Ok(DisplayObject::new(component).meta(meta))
	}
}
