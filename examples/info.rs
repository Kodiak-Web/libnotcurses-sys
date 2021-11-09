use libnotcurses_sys::*;

fn main() -> NcResult<()> {
    let nc = Nc::new_cli()?;
    let splane = nc.stdplane();
    splane.set_scrolling(true);

    putstrln!(splane, "ENVIRONMENT\n-----------")?;
    putstrln!(splane, "notcurses version: {}", Nc::version())?;
    putstrln!(splane, "terminal name: {}", nc.detected_terminal())?;
    putstrln!(splane, "user name: {}", Nc::accountname())?;
    putstrln!(splane, "host name: {}", Nc::hostname())?;
    putstrln!(splane)?;

    putstrln!(splane, "CAPABILITIES\n------------")?;
    putstrln!(
        splane,
        "Can display UTF-8: {0}
Can display braille characters: {1}
Can display sextant characters: {2}
Can display quadrant characters: {3}
Can display half block characters: {4}
Can open images: {5}
Can open videos: {6}
Supports Pixels: {7:?}
Supports True Color: {8}
Supports fading: {9}
Supports changing the palette: {10}
Palette size: {11:?}
",
        nc.canutf8(),
        nc.canbraille(),
        nc.cansextant(),
        nc.canquadrant(),
        nc.canhalfblock(),
        nc.canopen_images(),
        nc.canopen_videos(),
        nc.check_pixel_support(),
        nc.cantruecolor(),
        nc.canfade(),
        nc.canchangecolor(),
        nc.palette_size(),
    )?;

    putstrln!(splane, "GEOMETRY\n------------")?;
    let (t_rows, t_cols) = nc.term_dim_yx();
    putstrln!(
        splane,
        "Terminal dimensions: rows={0}, cols={1}",
        t_rows,
        t_cols
    )?;
    let pgeom = nc.stdplane().pixel_geom();
    putstrln!(splane, "{:#?}.", pgeom)?;

    let vg = nc.visual_geom(None, None)?;
    putstrln!(splane, "{:#?}.", vg)?;
    putstrln!(splane, "(blitter `{}` = {:?})", vg.blitter, vg.blitter_name())?;

    nc.render()?;
    nc.stop()?;
    Ok(())
}
