# S-stat markup language spec

S-stat defines markup using an S-expression style syntax to group elements
together.

Each element in an S-stat document is called a 'node'.
Nodes have the following syntax:

```lisp
(node-name [attributes...] children...)
```

## Attributes

Any node can have additional attributes added to it. Attributes must always
be specified immediately following the nodes name, and must be enclosed within
an attribute block (`[attributes...]`).

eg.

```lisp
(doc [
	(id main)
	(style (background-color black))
]
	(p [class simple-p] ;; a single attribute doesn't need to be parenthesized
		cool and interesting example paragraph
	)
	(p [style (color red)]
		other interesting paragraph but with fancy color
	)
)
```

Would produce the following HTML code
```html
<main id="main" class="__sstat-main" style="background-color: black">
		<p class="__sstat-paragraph simple-p">
			cool and interesting example paragraph
		</p>
		<p class="__sstat-paragraph" style="color:red">
			other intersting paragraph but with fancy color
		</p>
</main>
```

## Comments

Comments can be started using `;;` and will span until the end of their line

## `doc`

The root node of the content. This tag is required in any top-level S-stat
file.

Renders to an HTML `main` tag.

```lisp
(doc
	foo
)
```

Renders to:

```html
<main class="__sstat-main">
	<p class="__sstat-paragraph">foo</p>
</main>
```

## `sec`

A section of the document.

It's recommended for the first child of a sec node to be a [`title`](#title)
node.

```lisp
(sec
	(title cool and informative section)

	very informative paragraph
)
```

Renders to:

```html
<section class="__sstat-section">
	<h2 class="__sstat-title">cool and informative section</h2>

	<p class="__sstat-paragraph">very informative paragraph</p>
</section>
```

## `title`

An automatically incremented title/header tag.

The `title` node will produce an HTML `h1-6` tag, with the level of the `h` tag
being automatically determined based on how deeply nested this `title` tag is.
It, unlike the [`header`](#header) node, *will* appear in the table of
contents.

The content of the very first `title` node (the only `h1` element on the page)
will also be used as the page's title in the browser.

`title` nodes should probably be the first child of their parent node.

```lisp
(doc
	(title this will be an h1)

	(sec
		(title this an h2)

		(sec
			(title and this an h3)
		)
	)
)
```

Renders to:

```html
<main class="__sstat-main">
	<h1 class="__sstat-paragraph">this will be an h1</h1>

	<section class="__sstat-section">
		<h2 class="__sstat-title">this an h2<h2>

		<section class="__sstat-section">
			<h3 class="__sstat-title">and this an h3</h3>
		</section>
	</section>
</main>
```

## `p`

A paragraph.

```lisp
(p example paragraph)
```

Renders to:

```html
<p class="__sstat-paragraph">example paragraph</p>
```

## Lists

### `list`

An unordered list.

Any child nodes of the `list` should be [`li`](#li) nodes.

```lisp
(list
	(li
		first list item
		(header but also with a header)
	)
	(li second list item)
)
```

Renders to:

```html
<ul class="__sstat-unordered-list">
	<li class="__sstat-list-item">
		first list item
		<h2 class="__sstat-header">but also with a header</h2>
	</li>
	<li class="__sstat-list-item">second list item</li>
</ul>
```

### `ordlist`

An ordered list.

See [`list`](#list) for documentation.

### `li`

A list item.

Used to group different nodes together into a single list item.

See [`list`](#list) for examples.

### `dlist`, `dterm`, and `ddesc`

A description list.

The only allowed child nodes of a `dlist` node are `dterm` and `ddesc` nodes.

```lisp
(dlist
	(dterm first term)
	(ddesc description of the first term)

	(dterm second term)
	(ddesc description of the second term)
)
```

Renders to:

```html
<dl class="__sstat-description-list">
	<dt class="__sstat-description-term">first term</dt>
	<dd class="__sstat-description-description">description of the first term</dd>

	<dt class="__sstat-description-term">second term</dt>
	<dd class="__sstat-description-description">description of the second term</dd>
</dl>
```

## `header`

An automatically incremented title/header tag.

The `header` node will produce an HTML `h1-6` tag, with the level of the `h`
tag being automatically determined based on how deeply nested this `header` tag
is. It, unlike the [`title`](#title) node, *will not* appear in the table of
contents.

See [`title`](#title) for examples.

### `important`, `note`, `tip`, `caution`, and `warning`

Admonitions with various different stylings.

```lisp
(important an imporant piece of text)
```

Renders to:

```html
<div class="__sstat-admonition-important">
	<div class="__sstat-admonition-label">
		important
	</div>

	<div class="__sstat-admonition-content">
		an important piece of text
	</div>
</div>
```

Similar code is produced for the other admonition tags, but with the
`"admonition-important"` class and the content of the `"admonition-label"` div
changed to reflect the type of admonition.

## `table`

TODO

a table

## `image`

TODO

probably produce an HTML `figure` tag
will probably need `src`, `alt`, and maybe `caption` attributes

## `blockquote`

TODO

a block quote
will probably need a `source`/`cite` attribute

## `codeblock`

TODO

a codeblock
will probably need a `lang` attribute

## Footnotes

### `fnotes`

Renders all not-yet-shown footnotes at the current location.

```lisp
(sec
	(p
		this text references footnote 1 (fnote footnote 1)
	)

	(fnotes)
)
```

Renders to:

```html
<section class="__sstat-section">
	<p class="__sstat-paragraph">
		this text references footnote 1
		<sup class="__sstat-footnote-reference">
			<a href="#__sstat-footnote-definition-1">1</a>
		</sup>
	</p>

	<div class="__sstat-footnotes">
		<div class="__sstat-footnotes-header">Footnotes</span>
		<div class="__sstat-footnotes-definitions">
			<div
				id="__sstat-footnote-definition-1"
				class="__sstat-footnote-definition"
			>
				<sup class="__sstat-footnote-definition-id">1</sup>
				footnote 1
			</div>
		</div>
	</div>
</section>
```

### `fnote`

An inline footnote definition and reference

```lisp
(sec
	(p
		this text references footnote 1 (fnote footnote 1)
	)

	(fnotes)
)
```

Renders to:

```html
<section class="__sstat-section">
	<p class="__sstat-paragraph">
		this text references footnote 1
		<sup class="__sstat-footnote-reference">
			<a href="#__sstat-footnote-definition-1">1</a>
		</sup>
	</p>

	<div class="__sstat-footnotes">
		<div class="__sstat-footnotes-header">Footnotes</span>
		<div class="__sstat-footnotes-definitions">
			<div
				id="__sstat-footnote-definition-1"
				class="__sstat-footnote-definition"
			>
				<sup class="__sstat-footnote-definition-id">1</sup>
				footnote 1
			</div>
		</div>
	</div>
</section>
```

### `fnoteref` and `fnotedef`

Separated footnote references and definitions.

`fnoteref` has the following required attributes:
 - ref_id: The reference ID of the footnote that is being referenced

`fnotedef` has the following required attributes:
 - id: The unique ID of this footnote

```lisp
(sec
	this text references footnote 1 (fnoteref [ref_id 1])

	this text references footnote 2 (fnoteref [ref_id 2])

	(fnotedef [(id 1)]
		footnote 1
	)

	(fnotedef [(id 2)]
		footnote 2
	)

	(fnotes)
)
```

Renders to:

```html
<section class="__sstat-section">
	<p class="__sstat-paragraph">
		this text references footnote 1
		<sup class="__sstat-footnote-reference">
			<a href="#__sstat-footnote-definition-1">1</a>
		</sup>
	</p>

	<p class="__sstat-paragraph">
		this text references footnote 2
		<sup class="__sstat-footnote-reference">
			<a href="#__sstat-footnote-definition-2">2</a>
		</sup>
	</p>

	<div class="__sstat-footnotes">
		<div class="__sstat-footnotes-header">Footnotes</span>
		<div class="__sstat-footnotes-definitions">
			<div
				id="__sstat-footnote-definition-1"
				class="__sstat-footnote-definition"
			>
				<sup class="__sstat-footnote-definition-id">1</sup>
				footnote 1
			</div>
			<div
				id="__sstat-footnote-definition-2"
				class="__sstat-footnote-definition"
			>
				<sup class="__sstat-footnote-definition-id">2</sup>
				footnote 2
			</div>
		</div>
	</div>
</section>
```

## `div`

A block level div element.

Mostly used when needing to apply custom properties to a group of elements.

```lisp
(div [(class cool-custom-styles)]
	(p all of these paragraphs)
	(p will be styled with)
	(p cool custom styles)
)
```

Renders to:

```html
<div class="__sstat-div cool-custom-styles">
	<p>all of these paragraphs</p>
	<p>will be styled with</p>
	<p>cool custom styles</p>
</div>
```

See [`span`](#span) for an inline equivalent.

## `html`

A node containing raw HTML source code.

The HTML code within this node will not be processed in any way, and will be
included as-is in the final output.

```lisp
(html
	<div class="fancy-inline-html-div">
		<p>this HTML code will not</p>
		<p>be modified by the compiler</p>
	</div>
)
```

Renders to:

```html
<div class="fancy-inline-html-div">
	<p>this HTML code will not</p>
	<p>be modified by the compiler</p>
</div>
```

## Text Formatting

### `b`, `i`, and `mono`

Bold, italic, and monospace modifiers.

```lisp
(p
	a paragraph with (b (i bold, italic text)) and inline (mono monospace) text
)
```

Renders to:

```html
<p class="__sstat-paragraph">
	a paragraph with
		<b class="__sstat-bold">
			<i class="__sstat-italic">
				bold, italic text
			</i>
		</b>
	and inline
	<code class="__sstat-monospace">monospace</code> text
</p>
```

### `u`, `s`, and `mark`

Underline, strikethrough, and highlighted text

```lisp
(p
	a paragraph with (u underlined), (s strikethrough), and (mark highlighted)
	text.
)
```

Renders to:

```html
<p class="__sstat-paragraph">
	a paragraph with <u class="__sstat-underline">underlined</u>,
	<s class="__sstat-strikethrough">strikethrough</s>,
	and <mark class="__sstat-mark">highlighted</mark> text.
</p>
```

### `sub` and `sup`

Sub- and superscript

```lisp
(p
	a paragraph with (sub sub)script and (sup super)script.
)
```

Renders to:

```html
<p class="__sstat-paragraph">
	a paragraph with <sub class="__sstat-subscript">sub</sub>script
	and <sup class="__sstat-superscript">super</sub>script.
</p>
```

## `br`

A linebreak.

```lisp
(p
	a paragraph with a (br) linebreak in it
)
```

Renders to:

```html
<p>
	a paragraph with a <br /> linebreak in it
</p>
```

## `link`

A link to some remote resource.

Required attributes:
 - href: the URL of the resource

This node is intended to be used for remote resources (eg. other websites). It
will have a different class than the node intened to be used for
[cross-referencing](#ref) other nodes within the same document (eg. for a
table of contents)

```lisp
(link [(id example-link) (href https://example.com)] a link to example.com)
```

Renders to:

```html
<a id="example-link" class="__sstat-link" href="https://example.com">
	a link to example.com
</a>
```

## `ref`

A reference to some local node.

Required attributes:
 - ref_id: the ID of the node to reference

```lisp
(ref [(href #example-lin)] a link to the previous link)
```

Renders to:

```html
<a class="__sstat-ref" href="#example-link">a linkt to the previous link</a>
```
## `span`

An inline span element.

Mostly used when needing to apply custom properties to an inline element.

```lisp
(p this paragraph has some (span [(id funky)] funky) text in it)
```

Renders to:

```html
<p>
	this paragraph has some <span id="funky" class="__sstat-span">funky</span>
	text in it
</p>
```

See [`div`](#div) for a block-level equivalent.
