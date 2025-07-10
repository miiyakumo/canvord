pub fn preview_html(_html_output: String) -> String {
    format!(r#"<!DOCTYPE html>
               <html lang="cn">
               <head>
                   <meta charset="utf-8">
                   <title>Markdown Preview</title>
                   <meta name="viewport" content="width=device-width, initial-scale=1">
                   <style>
                       body {{
                           max-width: 768px;
                           margin: 2rem auto;
                           padding: 2rem;
                           font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", sans-serif;
                           line-height: 1.6;
                           color: #2e2e2e;
                           background-color: #fafafa;
                       }}
                       h1, h2, h3 {{
                           border-bottom: 1px solid #eaecef;
                           padding-bottom: 0.3em;
                           margin-top: 1.5em;
                       }}
                       pre, code {{
                           background-color: #f6f8fa;
                           padding: 0.2em 0.4em;
                           border-radius: 6px;
                           font-family: SFMono-Regular, Consolas, "Liberation Mono", Menlo, monospace;
                       }}
                       pre {{
                           padding: 1em;
                           overflow: auto;
                       }}
                       blockquote {{
                           color: #6a737d;
                           padding: 0 1em;
                           border-left: 0.25em solid #dfe2e5;
                       }}
                       ul {{
                           list-style: disc;
                           margin-left: 2em;
                       }}
                       table {{
                           border-collapse: collapse;
                       }}
                       th, td {{
                           border: 1px solid #dfe2e5;
                           padding: 6px 13px;
                       }}
                       img {{
                           max-width: 100%;
                       }}
                       @media (prefers-color-scheme: dark) {{
                           body {{
                               color: #d1d5db;
                               background-color: #1f2937;
                           }}
                           a {{ color: #93c5fd; }}
                           code, pre {{
                               background-color: #374151;
                               color: #f3f4f6;
                           }}
                       }}
                   </style>
               </head>
               <body>
                   {}
               </body>
               </html>"#, _html_output)
}
