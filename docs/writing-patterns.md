# Writing Regular Expressions to Detect Common XSS Tags

This guide will help you write regular expressions (regex) to detect common XSS (Cross-Site Scripting) vectors used in HTML. These vectors typically involve the use of HTML tags like `<script>`, `<iframe>`, and `<object>` to inject or execute harmful JavaScript code. Below are custom regex examples tailored to detect these patterns and others commonly used in XSS attacks.

## Basic Regex Patterns to Detect XSS Tags

### 1. Detecting `<script>` Tags

**Regex Pattern**:
```regex
(?i)<script[^>]*?>.*?</script>
```

### Explanation:
- `(?i)`: Case-insensitive flag to match both uppercase and lowercase tag names, such as `<SCRIPT>` or `<script>`.
- `<script`: Matches the opening `<script` tag.
- `[^>]*?`: Matches any characters except `>`, non-greedily, inside the `<script>` tag. This ensures it can handle various attributes or malformed tags.
- `.*?`: Matches any content inside the script tag (non-greedy).
- `</script>`: Matches the closing `</script>` tag.

### Purpose:
This regex detects the presence of `<script>` tags, which are often used in XSS attacks to inject malicious JavaScript code.

### 2. Detecting `<iframe>` Tags

**Regex Pattern**:
```regex
(?i)<iframe[^>]*?>.*?</iframe>
```

### Explanation:
- `(?i)`: Case-insensitive flag to match both uppercase and lowercase tag names, such as `<IFRAME>` or `<iframe>`.
- `<iframe`: Matches the opening `<iframe` tag.
- `[^>]*?`: Matches any characters (except `>`) within the `<iframe>` tag, non-greedily, allowing the matching of various attributes like `src`, `width`, etc.
- `.*?`: Matches any content inside the `<iframe>` tag.
- `</iframe>`: Matches the closing `</iframe>` tag.

### Purpose:
This regex identifies `<iframe>` tags, which are commonly used to embed external content, including malicious websites or exploitative scripts.

### 3. Detecting `<object>` Tags

**Regex Pattern**:
```regex
(?i)<object[^>]*?>.*?</object>
```

### Explanation:
- `(?i)`: Case-insensitive flag to match both uppercase and lowercase tag names, such as `<OBJECT>` or `<object>`.
- `<object`: Matches the opening `<object` tag.
- `[^>]*?`: Matches any characters except `>`, non-greedily, inside the `<object>` tag, handling various attributes.
- `.*?`: Matches any content inside the `<object>` tag.
- `</object>`: Matches the closing `</object>` tag.

### Purpose:
This regex detects the `<object>` tag, which can be exploited to embed potentially harmful plugins or files, like Flash objects, that may allow malicious scripts to run.

---

## Additional XSS Patterns

You can create more complex regex patterns to detect other potential XSS vectors by checking for attributes or specific tag behaviors that could indicate an attack.

### 4. Detecting `javascript:` in `href` or `src` Attributes

**Regex Pattern**:
```regex
(?i)<[a-z]+[^>]*?(href|src)\s*=\s*['\"]?javascript:[^'\">]+['\"]?[^>]*>
```

### Explanation:
- `(?i)`: Case-insensitive flag to match both uppercase and lowercase tag names.
- `<[a-z]+`: Matches any HTML tag, such as `<a>`, `<img>`, etc.
- `[^>]*?`: Matches any characters inside the tag, non-greedily, including attributes.
- `(href|src)`: Matches the `href` or `src` attributes, which can be used to execute malicious JavaScript.
- `\s*=\s*`: Matches the equal sign (`=`) with optional whitespace on either side.
- `['\"]?javascript:`: Detects the presence of `javascript:` in the attribute value, which is a common scheme for XSS.
- `[^'\">]+`: Matches any characters after `javascript:`, up until a `'`, `"`, or `>`.

### Purpose:
This regex detects tags with `href` or `src` attributes that use `javascript:` as a URL scheme, often found in links or images used to execute scripts.

### 5. Detecting Inline Event Handlers

**Regex Pattern**:
```regex
(?i)<[a-z]+[^>]*?(on[a-z]+)\s*=\s*['\"]?[^'\">]+['\"]?[^>]*>
```

### Explanation:
- `(?i)`: Case-insensitive flag to match HTML tag names.
- `<[a-z]+`: Matches any HTML tag.
- `[^>]*?`: Matches any characters inside the tag, non-greedily.
- `(on[a-z]+)`: Matches any inline event handler, such as `onclick`, `onload`, `onmouseover`, etc.
- `\s*=\s*`: Matches the equal sign with optional whitespace.
- `['\"]?`: Matches an optional single or double quote.
- `[^'\">]+`: Matches any characters after the `=` until a `'`, `"`, or `>`, indicating the event handler's JavaScript code.

### Purpose:
This regex identifies HTML tags with inline event handlers, which are often used to execute JavaScript code when triggered, making them a potential target for XSS attacks.

### Comments in pattern file

Here’s an improved version of the comments section:

#### Comments

- A comment line begins with `//`.
- Anything written after the `//` is treated as a comment and can be any text you like.
- Comments are ignored by the interpreter and do not affect the execution of the code.