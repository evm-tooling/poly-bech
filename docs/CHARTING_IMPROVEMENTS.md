# Charting Module Improvements & Feature Ideas

## 1. Existing Features That Could Be Improved/Extended

### Error Bars & Confidence Intervals
- **Current**: Shows CI 95% as simple vertical bars
- **Improvements**:
  - Support **different CI levels** (90%, 99%) via `ciLevel: 95`
  - Show **standard deviation bands** as shaded regions on line charts

### Regression Analysis
- **Current**: Auto-selects best model (O(1) through O(n³)) using BIC, shows dashed line + label
- **Improvements**:
  - Display **R² (coefficient of determination)** to show fit quality
  - Show **confidence bands** around regression line (prediction intervals)
  - Support **user-specified model** override: `regressionModel: "quadratic"`
  - Show **equation** with actual coefficients: `y = 2.3x² + 45x + 120`

### Axis Scaling
- **Current**: Linear and log scale
- **Improvements**:
  - **Symlog scale** (log scale that handles values near zero gracefully)
  - **Broken axis** for outliers (discontinuity indicator)
  - **Dual Y-axis** for comparing metrics with different scales (time vs memory)
  - **Percentage scale** option for relative comparisons

### Grid & Styling
- **Current**: Basic horizontal grid lines
- **Improvements**:
  - **Minor grid lines** (lighter, between major ticks)
  - **Vertical grid lines** option
  - **Annotation support** (add text labels at specific points)

---

## 2. New Features Common in Professional Benchmarking Tools

### Throughput & Scaling Charts

#### Speedup Charts
```
charting.drawSpeedupChart(
    title: "Go vs TypeScript Speedup",
    output: "speedup.svg",
    baseline: "typescript"
)
```
- Shows relative performance (2x faster, 0.5x slower)
- Useful for "how much faster is X than Y" questions

#### Scaling Efficiency
```
charting.drawScalingChart(
    title: "Parallel Scaling Efficiency",
    output: "scaling.svg",
    idealLine: true  // Show linear scaling reference
)
```
- Shows how performance scales with input size
- Ideal line overlay for O(n) reference
- **Amdahl's Law** visualization for parallel benchmarks

#### Data Tables
```
charting.drawTable(
    title: "Detailed Results",
    output: "results-table.svg",
    columns: ["benchmark", "go_time", "ts_time", "speedup", "winner"]
)
```
- Sortable data tables as SVG
- Conditional formatting (color-code winners)

---
