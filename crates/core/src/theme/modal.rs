use iced_aw::style::modal::Appearance;

impl iced_aw::modal::StyleSheet for super::Theme {
     type Style = ();
     fn active(&self, style: Self::Style) -> Appearance {
        Appearance::default() 
     }
}
