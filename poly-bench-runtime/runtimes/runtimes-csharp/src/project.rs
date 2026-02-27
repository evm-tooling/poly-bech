//! C# project root detection.

use poly_bench_dsl::Lang;
use poly_bench_traits::ProjectRootDetector;

pub struct CSharpDetector;

impl ProjectRootDetector for CSharpDetector {
    fn lang(&self) -> Lang {
        Lang::CSharp
    }

    fn marker_files(&self) -> &[&'static str] {
        &["polybench.csproj", ".sln", "Directory.Build.props", "global.json"]
    }
}

pub static CSHARP_DETECTOR: CSharpDetector = CSharpDetector;
