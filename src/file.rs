use std::{
    borrow::Cow,
    fs,
    io::{self, stdout, Write},
    path::Path,
};

/// Used for specifying how large an image should be rendered
#[derive(Debug, Copy, Clone)]
pub enum Dimension {
    /// iterm will choose a size for you
    Auto,
    /// the amount of pixels that will be used
    Pixel(u32),
    /// the amount of cells that will be used
    Cells(u32),
    /// percent of the current terminal size
    Percent(u8),
}
impl std::fmt::Display for Dimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Dimension::Auto => write!(f, "auto"),
            Dimension::Pixel(val) => write!(f, "{}px", val),
            Dimension::Cells(val) => write!(f, "{}", val),
            Dimension::Percent(val) => {
                assert!(*val <= 100, "percent cannot be greater than 100");
                write!(f, "{}%", val)
            }
        }
    }
}

/// A builder for drawing images in the terminal
/// ```rust,no_run
/// use iterm420::{Dimension, File};
///
/// File::read("path/to/some/image.png")?
///     .height(Dimension::Cells(14))
///     .width(Dimension::Percent(100))
///     .preserve_aspect_ratio(false)
///     .show()?;
/// # Ok::<_, std::io::Error>(())
/// ```
#[derive(Default)]
pub struct File<'c> {
    name: Option<String>,
    size: Option<u32>,
    width: Option<Dimension>,
    height: Option<Dimension>,
    preserve_aspect_ratio: Option<bool>,
    contents: Cow<'c, [u8]>,
}
impl<'c> File<'c> {
    /// creates a new image from its content
    pub fn new(contents: &'c [u8]) -> Self {
        Self {
            contents: contents.into(),
            ..Default::default()
        }
    }

    /// creates a new image from the given file
    pub fn read(path: impl AsRef<Path>) -> io::Result<Self> {
        let contents = fs::read(path)?;
        Ok(Self {
            contents: contents.into(),
            ..Default::default()
        })
    }

    /// Set the name of the file, only used with [`download`](struct.File.html#method.download).
    ///
    /// Default is "Unnamed file".
    pub fn name(&mut self, name: String) -> &mut Self {
        self.name = Some(name);
        self
    }
    /// Set the size of the file in bytes.
    /// It will be used with [`download`](struct.File.html#method.download) for showing the progress indicator.
    pub fn size(&mut self, size: u32) -> &mut Self {
        self.size = Some(size);
        self
    }
    /// Set the width of the image
    pub fn width(&mut self, width: Dimension) -> &mut Self {
        self.width = Some(width);
        self
    }
    /// Set the height of the image
    pub fn height(&mut self, height: Dimension) -> &mut Self {
        self.height = Some(height);
        self
    }
    /// Specifies, whether the aspect ratio of the image should be kept
    pub fn preserve_aspect_ratio(&mut self, preserve: bool) -> &mut Self {
        self.preserve_aspect_ratio = Some(preserve);
        self
    }

    fn action(&self, inline: bool, img_data: &[u8]) -> io::Result<()> {
        write!(stdout(), "\x1b]1337;File=")?;
        if let Some(name) = &self.name {
            write!(stdout(), "name={};", name)?;
        }
        if let Some(size) = &self.size {
            write!(stdout(), "size={};", size)?;
        }
        if let Some(width) = &self.width {
            write!(stdout(), "width={};", width)?;
        }
        if let Some(height) = &self.height {
            write!(stdout(), "height={};", height)?;
        }
        if let Some(ar) = self.preserve_aspect_ratio {
            write!(stdout(), "preserveAspectRatio={};", ar as u8)?;
        }
        write!(stdout(), "inline={};", inline as u8)?;

        write!(stdout(), ":{}\x07", base64::encode(img_data))?;
        Ok(())
    }

    /// Download the file
    pub fn download(&self) -> io::Result<()> {
        self.action(false, &self.contents)
    }

    /// Display the image in the terminal.
    pub fn show(&self) -> io::Result<()> {
        self.action(true, &self.contents)?;
        writeln!(stdout())
    }
}
