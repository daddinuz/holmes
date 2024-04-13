pub trait City {}

pub struct NewYork(());
pub struct Odense(());
pub struct Rome(());
pub struct Sydney(());
pub struct Tokyo(());

impl City for NewYork {}
impl City for Odense {}
impl City for Rome {}
impl City for Sydney {}
impl City for Tokyo {}
