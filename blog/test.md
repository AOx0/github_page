# Test

## Test 

### Test 

This **is** a *test*


```rust 
fn main() -> anyhow::Result<()> {
  println!("Hello to the test");
  Ok(())
}
```

Here the `main` function prints the string `Hello to the test` to the console.

Here is another block of code:

```rust
impl Default for Car<Off> {
    fn default() -> Car<Off> {
        Self {
            speed: 0,
            stage: PhantomData,
        }
    }
}
```

```python
def hola():
  print("Hello World")
```

<pre><code class="code-highlighted code-mathematica"><span class="syntax-all syntax-variable">result</span> <span class="syntax-all syntax-keyword">=</span> <span class="syntax-all syntax-keyword">&lt;||&gt;</span>
<span class="syntax-all syntax-constant">Do</span>[
	<span class="syntax-all syntax-variable">file</span> <span class="syntax-all syntax-keyword">=</span> <span class="syntax-all syntax-string">&quot;~/Downloads/covid/covid_&quot;</span><span class="syntax-all syntax-keyword">&lt;&gt;</span><span class="syntax-all syntax-constant">ToString</span><span class="syntax-all syntax-keyword">@</span><span class="syntax-all syntax-variable">file</span><span class="syntax-all syntax-keyword">&lt;&gt;</span><span class="syntax-all syntax-string">&quot;.csv&quot;</span><span class="syntax-all syntax-keyword">;</span>
	<span class="syntax-all syntax-constant">Print</span>[<span class="syntax-all syntax-string">&quot;Loading... &quot;</span><span class="syntax-all syntax-keyword">&lt;&gt;</span><span class="syntax-all syntax-variable">file</span>]<span class="syntax-all syntax-keyword">;</span>
	<span class="syntax-all syntax-variable">data</span> <span class="syntax-all syntax-keyword">=</span> <span class="syntax-all syntax-constant">Import</span>[<span class="syntax-all syntax-variable">file</span>, {<span class="syntax-all syntax-string">&quot;CSV&quot;</span>, <span class="syntax-all syntax-string">&quot;Dataset&quot;</span>}, <span class="syntax-all syntax-string">&quot;HeaderLines&quot;</span><span class="syntax-all syntax-keyword">-&gt;</span><span class="syntax-all syntax-constant">1</span>]<span class="syntax-all syntax-keyword">;</span>
	<span class="syntax-all syntax-variable">dates</span> <span class="syntax-all syntax-keyword">=</span> <span class="syntax-all syntax-entity">Counts</span>[<span class="syntax-all syntax-constant">Normal</span><span class="syntax-all syntax-keyword">@</span><span class="syntax-all syntax-entity">data</span>[<span class="syntax-all syntax-constant">All</span>, <span class="syntax-all syntax-string">&quot;FECHA_INGRESO&quot;</span>]]<span class="syntax-all syntax-keyword">;</span>
	<span class="syntax-all syntax-variable">result</span> <span class="syntax-all syntax-keyword">=</span> <span class="syntax-all syntax-entity">Merge</span>[{<span class="syntax-all syntax-variable">result</span>, <span class="syntax-all syntax-variable">dates</span>}, <span class="syntax-all syntax-constant">Total</span>],
{<span class="syntax-all syntax-variable">file</span>, <span class="syntax-all syntax-constant">1</span>, <span class="syntax-all syntax-constant">101</span>}]

<span class="syntax-all syntax-variable">dates</span> <span class="syntax-all syntax-keyword">=</span> <span class="syntax-all syntax-constant">Flatten</span> <span class="syntax-all syntax-keyword">/@</span> <span class="syntax-all syntax-constant">List</span> <span class="syntax-all syntax-keyword">@@@</span> <span class="syntax-all syntax-constant">Normal</span> <span class="syntax-all syntax-keyword">@</span> <span class="syntax-all syntax-variable">result</span><span class="syntax-all syntax-keyword">;</span>
<span class="syntax-all syntax-constant">Do</span>[<span class="syntax-all syntax-variable">dates</span>[[<span class="syntax-all syntax-variable">n</span>]][[<span class="syntax-all syntax-constant">1</span>]] <span class="syntax-all syntax-keyword">=</span> <span class="syntax-all syntax-entity">FromDateString</span>[<span class="syntax-all syntax-variable">dates</span>[[<span class="syntax-all syntax-variable">n</span>]][[<span class="syntax-all syntax-constant">1</span>]]], {<span class="syntax-all syntax-variable">n</span>, <span class="syntax-all syntax-constant">1</span>, <span class="syntax-all syntax-constant">Length</span>[<span class="syntax-all syntax-variable">dates</span>]}]<span class="syntax-all syntax-keyword">;</span>

<span class="syntax-all syntax-constant">DateListPlot</span>[<span class="syntax-all syntax-variable">dates</span>, <span class="syntax-all syntax-constant">PlotRange</span><span class="syntax-all syntax-keyword">-&gt;</span>{{<span class="syntax-all syntax-constant">Automatic</span>, <span class="syntax-all syntax-entity">FromDateString</span>[<span class="syntax-all syntax-string">&quot;2021-12-14&quot;</span>]}, <span class="syntax-all syntax-constant">Automatic</span>}, <span class="syntax-all syntax-constant">ImageSize</span><span class="syntax-all syntax-keyword">-&gt;</span><span class="syntax-all syntax-constant">Full</span>]</code></pre>
