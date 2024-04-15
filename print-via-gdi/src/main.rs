use windows::core::{w, PCWSTR};
use windows::Win32::Graphics::Gdi::{
    CreateDCW, CreateFontW, DeleteDC, Ellipse, GetDeviceCaps, GetTextMetricsW, LineTo, MoveToEx,
    SelectObject, TextOutW, CLIP_DEFAULT_PRECIS, DEFAULT_QUALITY, FF_ROMAN, FW_BOLD,
    GET_DEVICE_CAPS_INDEX, HDC, HORZRES, OUT_DEFAULT_PRECIS, SHIFTJIS_CHARSET, TEXTMETRICW,
    VARIABLE_PITCH, VERTRES,
};
use windows::Win32::Storage::Xps::{EndDoc, EndPage, StartDocW, StartPage, DOCINFOW};

use compat::to_wstring;

fn main() {
    unsafe {
        // 物理プリンタに印刷する場合は、プリンタ名とドライバ名を良い感じに設定する。
        let printer_name = w!("Microsoft Print To PDF");
        let driver_name = w!("Microsoft Print To PDF");

        let hdc = CreateDCW(driver_name, printer_name, None, None);

        let mut doc_info = DOCINFOW {
            cbSize: std::mem::size_of::<DOCINFOW>() as i32,
            lpszDocName: PCWSTR(w!("Print by rust").as_ptr()),
            ..Default::default()
        };
        doc_info.cbSize = std::mem::size_of::<DOCINFOW>() as i32;
        doc_info.lpszDocName = PCWSTR(w!("Print by rust").as_ptr());

        StartDocW(hdc, &doc_info);
        StartPage(hdc);

        let mut text_metric = TEXTMETRICW::default();

        let font1 = CreateFontW(
            200,
            0,
            0,
            0,
            FW_BOLD.0 as i32,
            1,
            1,
            0,
            SHIFTJIS_CHARSET.0.into(),
            OUT_DEFAULT_PRECIS.0.into(),
            CLIP_DEFAULT_PRECIS.0.into(),
            DEFAULT_QUALITY.0.into(),
            (VARIABLE_PITCH.0 | FF_ROMAN.0).into(),
            None,
        );

        let font2 = CreateFontW(
            200,
            0,
            3150,
            0,
            FW_BOLD.0 as i32,
            0,
            0,
            0,
            SHIFTJIS_CHARSET.0.into(),
            OUT_DEFAULT_PRECIS.0.into(),
            CLIP_DEFAULT_PRECIS.0.into(),
            DEFAULT_QUALITY.0.into(),
            (VARIABLE_PITCH.0 | FF_ROMAN.0).into(),
            w!("HGP創英角ﾎﾟｯﾌﾟ体"),
        );

        let _ = GetTextMetricsW(hdc, &mut text_metric);
        SelectObject(hdc, font1);
        let _ = TextOutW(hdc, 200, 200, &to_wstring("これはテスト印刷です"));
        SelectObject(hdc, font2);
        let _ = TextOutW(hdc, 300, 600, &to_wstring("HGP創英角ﾎﾟｯﾌﾟ体"));

        paint_bmp(hdc);

        EndPage(hdc);
        EndDoc(hdc);

        let _ = DeleteDC(hdc);
    }
}

fn paint_bmp(hdc: HDC) {
    unsafe {
        let wx = GetDeviceCaps(hdc, GET_DEVICE_CAPS_INDEX(HORZRES.0)) - 100;
        let wy = GetDeviceCaps(hdc, GET_DEVICE_CAPS_INDEX(VERTRES.0)) - 100;

        for i in 1000..wy {
            if i % 200 == 0 {
                let _ = MoveToEx(hdc, 100, i, None);
                let _ = LineTo(hdc, wx, i);
            }
        }

        for i in 100..wx {
            if i % 200 == 0 {
                let _ = MoveToEx(hdc, i, 1000, None);
                let _ = LineTo(hdc, i, wy);
            }
        }

        let _ = Ellipse(hdc, 1250, 1250, 1650, 1650);
    }
}
