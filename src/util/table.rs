use anyhow::Result;
use docx_rs::*;
use std::fs::File;
use std::io::Read;

pub fn gen_table_header(cells: Vec<&str>) -> TableRow {
    let mut one_row = vec![];
    for cell in cells {
        one_row.push(
            TableCell::new().add_paragraph(
                Paragraph::new().add_run(
                    Run::new()
                        .add_text(cell)
                        .size(20)
                        .bold()
                        .fonts(RunFonts::new().ascii("Arial")),
                ),
            ),
        );
    }
    return TableRow::new(one_row);
}

pub fn gen_table_row(cells: Vec<&String>, font_size: usize, color: &str) -> TableRow {
    let mut one_row = vec![];
    for cell in cells {
        one_row.push(TableCell::new().add_paragraph(
            Paragraph::new().add_run(Run::new().add_text(cell).color(color).size(font_size)),
        ));
    }
    return TableRow::new(one_row);
}

pub fn gen_table(
    tb_header: TableRow,
    tb_rows: &mut Vec<TableRow>,
    grid: Vec<usize>,
    layout: TableLayoutType,
    indent: i32,
) -> Table {
    // generate table template
    let mut table = vec![];
    table.append(&mut vec![tb_header]);
    table.append(tb_rows);

    return Table::new(table)
        .set_grid(grid)
        .layout(layout)
        .indent(indent);
}

pub fn gen_heading(txt: &str, txt_size: usize, level_size: usize) -> Paragraph {
    return Paragraph::new()
        .add_run(
            Run::new()
                .add_text(txt)
                .size(txt_size)
                .bold()
                .fonts(RunFonts::new().ascii("Arial")),
        )
        .outline_lvl(level_size);
}

pub fn gen_text(txt: &str, size: usize, color: &str) -> Paragraph {
    return Paragraph::new()
        .add_run(
            Run::new()
                .add_text(txt)
                .size(size)
                .color(color)
                .fonts(RunFonts::new().ascii("Arial")),
        )
        .indent(Some(200), None, None, None);
}

pub enum DocType {
    Patagraph(Paragraph),
    Table(Table),
}

pub fn gen_image(image_name: String) -> Result<Paragraph> {
    let mut img = File::open(format!("/tmp/ticheck_image_dir/{}.png", &image_name))?;
    let mut buf = Vec::new();
    let _ = img.read_to_end(&mut buf)?;
    let pic = Pic::new(buf).size(500, 250);
    Ok(Paragraph::new()
        .add_run(Run::new().add_image(pic))
        .indent(Some(200), None, None, None))
}

pub fn gen_docx(docx_path: &str, docx: &mut Docx) -> Result<(), DocxError> {
    let path = std::path::Path::new(docx_path);
    let file = std::fs::File::create(&path).unwrap();
    docx.build().pack(file)?;
    Ok(())
}
