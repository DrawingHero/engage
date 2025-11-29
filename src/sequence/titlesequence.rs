#[repr(C)]
#[unity::class("App", "TitleSequence")]
pub struct TitleSequence {
    // Start SingletonProcInst here
    pub proc: ProcInstFields,
    is_resume: bool,
    is_loaded: bool,
    // End here
}

#[repr(C)]
pub struct TitleSequenceStaticFields {
    pub jump_label: TitleSequenceLabel,
    pub fake_label: i32,
    pub initialized: bool,
}
