(function() {var implementors = {
"palette":[["impl&lt;Wp, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html\" title=\"trait core::ops::arith::MulAssign\">MulAssign</a>&lt;<a class=\"struct\" href=\"palette/struct.Xyz.html\" title=\"struct palette::Xyz\">Xyz</a>&lt;Wp, T&gt;&gt; for <a class=\"struct\" href=\"palette/struct.Xyz.html\" title=\"struct palette::Xyz\">Xyz</a>&lt;Wp, T&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"palette/trait.FloatComponent.html\" title=\"trait palette::FloatComponent\">FloatComponent</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html\" title=\"trait core::ops::arith::MulAssign\">MulAssign</a>,\n    Wp: <a class=\"trait\" href=\"palette/white_point/trait.WhitePoint.html\" title=\"trait palette::white_point::WhitePoint\">WhitePoint</a>,</span>"],["impl&lt;S, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html\" title=\"trait core::ops::arith::MulAssign\">MulAssign</a>&lt;<a class=\"struct\" href=\"palette/luma/struct.Luma.html\" title=\"struct palette::luma::Luma\">Luma</a>&lt;S, T&gt;&gt; for <a class=\"struct\" href=\"palette/luma/struct.Luma.html\" title=\"struct palette::luma::Luma\">Luma</a>&lt;S, T&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"palette/trait.Component.html\" title=\"trait palette::Component\">Component</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html\" title=\"trait core::ops::arith::MulAssign\">MulAssign</a>,\n    S: <a class=\"trait\" href=\"palette/luma/trait.LumaStandard.html\" title=\"trait palette::luma::LumaStandard\">LumaStandard</a>&lt;TransferFn = <a class=\"struct\" href=\"palette/encoding/linear/struct.LinearFn.html\" title=\"struct palette::encoding::linear::LinearFn\">LinearFn</a>&gt;,</span>"],["impl&lt;T: <a class=\"trait\" href=\"palette/float/trait.Float.html\" title=\"trait palette::float::Float\">Float</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html\" title=\"trait core::ops::arith::MulAssign\">MulAssign</a>, C: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html\" title=\"trait core::ops::arith::MulAssign\">MulAssign</a>&lt;T&gt;&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html\" title=\"trait core::ops::arith::MulAssign\">MulAssign</a>&lt;T&gt; for <a class=\"struct\" href=\"palette/blend/struct.PreAlpha.html\" title=\"struct palette::blend::PreAlpha\">PreAlpha</a>&lt;C, T&gt;"],["impl&lt;S, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html\" title=\"trait core::ops::arith::MulAssign\">MulAssign</a>&lt;<a class=\"struct\" href=\"palette/rgb/struct.Rgb.html\" title=\"struct palette::rgb::Rgb\">Rgb</a>&lt;S, T&gt;&gt; for <a class=\"struct\" href=\"palette/rgb/struct.Rgb.html\" title=\"struct palette::rgb::Rgb\">Rgb</a>&lt;S, T&gt;<span class=\"where fmt-newline\">where\n    S: <a class=\"trait\" href=\"palette/rgb/trait.RgbStandard.html\" title=\"trait palette::rgb::RgbStandard\">RgbStandard</a>&lt;TransferFn = <a class=\"struct\" href=\"palette/encoding/linear/struct.LinearFn.html\" title=\"struct palette::encoding::linear::LinearFn\">LinearFn</a>&gt;,\n    T: <a class=\"trait\" href=\"palette/trait.Component.html\" title=\"trait palette::Component\">Component</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html\" title=\"trait core::ops::arith::MulAssign\">MulAssign</a>,</span>"],["impl&lt;Wp, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html\" title=\"trait core::ops::arith::MulAssign\">MulAssign</a>&lt;<a class=\"struct\" href=\"palette/struct.Yxy.html\" title=\"struct palette::Yxy\">Yxy</a>&lt;Wp, T&gt;&gt; for <a class=\"struct\" href=\"palette/struct.Yxy.html\" title=\"struct palette::Yxy\">Yxy</a>&lt;Wp, T&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"palette/trait.FloatComponent.html\" title=\"trait palette::FloatComponent\">FloatComponent</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html\" title=\"trait core::ops::arith::MulAssign\">MulAssign</a>,\n    Wp: <a class=\"trait\" href=\"palette/white_point/trait.WhitePoint.html\" title=\"trait palette::white_point::WhitePoint\">WhitePoint</a>,</span>"],["impl&lt;C: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html\" title=\"trait core::ops::arith::MulAssign\">MulAssign</a>, T: <a class=\"trait\" href=\"palette/float/trait.Float.html\" title=\"trait palette::float::Float\">Float</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html\" title=\"trait core::ops::arith::MulAssign\">MulAssign</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html\" title=\"trait core::ops::arith::MulAssign\">MulAssign</a>&lt;<a class=\"struct\" href=\"palette/blend/struct.PreAlpha.html\" title=\"struct palette::blend::PreAlpha\">PreAlpha</a>&lt;C, T&gt;&gt; for <a class=\"struct\" href=\"palette/blend/struct.PreAlpha.html\" title=\"struct palette::blend::PreAlpha\">PreAlpha</a>&lt;C, T&gt;"],["impl&lt;S, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html\" title=\"trait core::ops::arith::MulAssign\">MulAssign</a>&lt;T&gt; for <a class=\"struct\" href=\"palette/luma/struct.Luma.html\" title=\"struct palette::luma::Luma\">Luma</a>&lt;S, T&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"palette/trait.Component.html\" title=\"trait palette::Component\">Component</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html\" title=\"trait core::ops::arith::MulAssign\">MulAssign</a>,\n    S: <a class=\"trait\" href=\"palette/luma/trait.LumaStandard.html\" title=\"trait palette::luma::LumaStandard\">LumaStandard</a>&lt;TransferFn = <a class=\"struct\" href=\"palette/encoding/linear/struct.LinearFn.html\" title=\"struct palette::encoding::linear::LinearFn\">LinearFn</a>&gt;,</span>"],["impl&lt;C: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html\" title=\"trait core::ops::arith::MulAssign\">MulAssign</a>, T: <a class=\"trait\" href=\"palette/float/trait.Float.html\" title=\"trait palette::float::Float\">Float</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html\" title=\"trait core::ops::arith::MulAssign\">MulAssign</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html\" title=\"trait core::ops::arith::MulAssign\">MulAssign</a>&lt;<a class=\"struct\" href=\"palette/struct.Alpha.html\" title=\"struct palette::Alpha\">Alpha</a>&lt;C, T&gt;&gt; for <a class=\"struct\" href=\"palette/struct.Alpha.html\" title=\"struct palette::Alpha\">Alpha</a>&lt;C, T&gt;"],["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html\" title=\"trait core::ops::arith::MulAssign\">MulAssign</a>&lt;<a class=\"struct\" href=\"palette/struct.Oklab.html\" title=\"struct palette::Oklab\">Oklab</a>&lt;T&gt;&gt; for <a class=\"struct\" href=\"palette/struct.Oklab.html\" title=\"struct palette::Oklab\">Oklab</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"palette/trait.FloatComponent.html\" title=\"trait palette::FloatComponent\">FloatComponent</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html\" title=\"trait core::ops::arith::MulAssign\">MulAssign</a>,</span>"],["impl&lt;Wp, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html\" title=\"trait core::ops::arith::MulAssign\">MulAssign</a>&lt;<a class=\"struct\" href=\"palette/struct.Lab.html\" title=\"struct palette::Lab\">Lab</a>&lt;Wp, T&gt;&gt; for <a class=\"struct\" href=\"palette/struct.Lab.html\" title=\"struct palette::Lab\">Lab</a>&lt;Wp, T&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"palette/trait.FloatComponent.html\" title=\"trait palette::FloatComponent\">FloatComponent</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html\" title=\"trait core::ops::arith::MulAssign\">MulAssign</a>,\n    Wp: <a class=\"trait\" href=\"palette/white_point/trait.WhitePoint.html\" title=\"trait palette::white_point::WhitePoint\">WhitePoint</a>,</span>"],["impl&lt;Wp, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html\" title=\"trait core::ops::arith::MulAssign\">MulAssign</a>&lt;T&gt; for <a class=\"struct\" href=\"palette/struct.Yxy.html\" title=\"struct palette::Yxy\">Yxy</a>&lt;Wp, T&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"palette/trait.FloatComponent.html\" title=\"trait palette::FloatComponent\">FloatComponent</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html\" title=\"trait core::ops::arith::MulAssign\">MulAssign</a>,\n    Wp: <a class=\"trait\" href=\"palette/white_point/trait.WhitePoint.html\" title=\"trait palette::white_point::WhitePoint\">WhitePoint</a>,</span>"],["impl&lt;S, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html\" title=\"trait core::ops::arith::MulAssign\">MulAssign</a>&lt;T&gt; for <a class=\"struct\" href=\"palette/rgb/struct.Rgb.html\" title=\"struct palette::rgb::Rgb\">Rgb</a>&lt;S, T&gt;<span class=\"where fmt-newline\">where\n    S: <a class=\"trait\" href=\"palette/rgb/trait.RgbStandard.html\" title=\"trait palette::rgb::RgbStandard\">RgbStandard</a>&lt;TransferFn = <a class=\"struct\" href=\"palette/encoding/linear/struct.LinearFn.html\" title=\"struct palette::encoding::linear::LinearFn\">LinearFn</a>&gt;,\n    T: <a class=\"trait\" href=\"palette/trait.Component.html\" title=\"trait palette::Component\">Component</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html\" title=\"trait core::ops::arith::MulAssign\">MulAssign</a>,</span>"],["impl&lt;Wp, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html\" title=\"trait core::ops::arith::MulAssign\">MulAssign</a>&lt;T&gt; for <a class=\"struct\" href=\"palette/struct.Luv.html\" title=\"struct palette::Luv\">Luv</a>&lt;Wp, T&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"palette/trait.FloatComponent.html\" title=\"trait palette::FloatComponent\">FloatComponent</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html\" title=\"trait core::ops::arith::MulAssign\">MulAssign</a>,\n    Wp: <a class=\"trait\" href=\"palette/white_point/trait.WhitePoint.html\" title=\"trait palette::white_point::WhitePoint\">WhitePoint</a>,</span>"],["impl&lt;Wp, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html\" title=\"trait core::ops::arith::MulAssign\">MulAssign</a>&lt;T&gt; for <a class=\"struct\" href=\"palette/struct.Lab.html\" title=\"struct palette::Lab\">Lab</a>&lt;Wp, T&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"palette/trait.FloatComponent.html\" title=\"trait palette::FloatComponent\">FloatComponent</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html\" title=\"trait core::ops::arith::MulAssign\">MulAssign</a>,\n    Wp: <a class=\"trait\" href=\"palette/white_point/trait.WhitePoint.html\" title=\"trait palette::white_point::WhitePoint\">WhitePoint</a>,</span>"],["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html\" title=\"trait core::ops::arith::MulAssign\">MulAssign</a>&lt;T&gt; for <a class=\"struct\" href=\"palette/struct.Oklab.html\" title=\"struct palette::Oklab\">Oklab</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"palette/trait.FloatComponent.html\" title=\"trait palette::FloatComponent\">FloatComponent</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html\" title=\"trait core::ops::arith::MulAssign\">MulAssign</a>,</span>"],["impl&lt;Wp, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html\" title=\"trait core::ops::arith::MulAssign\">MulAssign</a>&lt;T&gt; for <a class=\"struct\" href=\"palette/struct.Xyz.html\" title=\"struct palette::Xyz\">Xyz</a>&lt;Wp, T&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"palette/trait.FloatComponent.html\" title=\"trait palette::FloatComponent\">FloatComponent</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html\" title=\"trait core::ops::arith::MulAssign\">MulAssign</a>,\n    Wp: <a class=\"trait\" href=\"palette/white_point/trait.WhitePoint.html\" title=\"trait palette::white_point::WhitePoint\">WhitePoint</a>,</span>"],["impl&lt;Wp, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html\" title=\"trait core::ops::arith::MulAssign\">MulAssign</a>&lt;<a class=\"struct\" href=\"palette/struct.Luv.html\" title=\"struct palette::Luv\">Luv</a>&lt;Wp, T&gt;&gt; for <a class=\"struct\" href=\"palette/struct.Luv.html\" title=\"struct palette::Luv\">Luv</a>&lt;Wp, T&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"palette/trait.FloatComponent.html\" title=\"trait palette::FloatComponent\">FloatComponent</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html\" title=\"trait core::ops::arith::MulAssign\">MulAssign</a>,\n    Wp: <a class=\"trait\" href=\"palette/white_point/trait.WhitePoint.html\" title=\"trait palette::white_point::WhitePoint\">WhitePoint</a>,</span>"],["impl&lt;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html\" title=\"trait core::ops::arith::MulAssign\">MulAssign</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>, C: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html\" title=\"trait core::ops::arith::MulAssign\">MulAssign</a>&lt;T&gt;&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html\" title=\"trait core::ops::arith::MulAssign\">MulAssign</a>&lt;T&gt; for <a class=\"struct\" href=\"palette/struct.Alpha.html\" title=\"struct palette::Alpha\">Alpha</a>&lt;C, T&gt;"]],
"sk6812_rpi":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html\" title=\"trait core::ops::arith::MulAssign\">MulAssign</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>&gt; for <a class=\"struct\" href=\"sk6812_rpi/led/struct.Led.html\" title=\"struct sk6812_rpi::led::Led\">Led</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html\" title=\"trait core::ops::arith::MulAssign\">MulAssign</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.f32.html\">f32</a>&gt; for <a class=\"struct\" href=\"sk6812_rpi/led/struct.Led.html\" title=\"struct sk6812_rpi::led::Led\">Led</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html\" title=\"trait core::ops::arith::MulAssign\">MulAssign</a>&lt;<a class=\"struct\" href=\"sk6812_rpi/led/struct.Led.html\" title=\"struct sk6812_rpi::led::Led\">Led</a>&gt; for <a class=\"struct\" href=\"sk6812_rpi/led/struct.Led.html\" title=\"struct sk6812_rpi::led::Led\">Led</a>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()