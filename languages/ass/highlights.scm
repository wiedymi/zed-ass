; ASS Subtitle Format Highlighting

; Section headers like [Script Info], [V4+ Styles], [Events]
(section_header) @tag

; Key-value pairs in sections
(key) @property
(value) @string

; Comments (lines starting with ;)
(comment) @comment

; Style definitions
(style_name) @type
(style_property) @attribute

; Event types
(event_type) @keyword

; Dialogue text
(dialogue_text) @string

; Time codes
(time_code) @number

; Style override tags in dialogue
(style_override) @function

; Effect names
(effect) @function.special

; Numbers (font size, positions, etc.)
(number) @number

; Colors and alpha values
(color) @constant
(alpha) @constant

; Actor names
(actor) @variable

; Punctuation
"[" @punctuation.bracket
"]" @punctuation.bracket
"=" @operator
"," @punctuation.delimiter
":" @punctuation.delimiter