// // ModeSpinner renders a Spinner
// const ModeSpinner: usize = 1;
// // ModeProgress renders a ProgressBar. Not supported yet.
// const ModeProgress: usize = 2;
// // ModeDone renders as "Done" message.
// const ModeDone: usize = 3;
// // ModeError renders as "Error" message.
// const ModeError: usize = 4;

// DisplayProps controls the display of the progress bar.
pub struct DisplayProps {
    pub prefix: String,
    pub suffix: String, // If `Mode == Done / Error`, Suffix is not printed
    pub mode: usize,
}
