-- Pandoc Lua filter used when converting the book's Markdown to Typst.
--
--  * GitHub alerts (> [!NOTE] ...)      -> #callout(kind: "note")[...]
--  * a paragraph holding only an image  -> #figure(image(...), caption: [...])
--    with a sensible width (screenshots: 92%; SVG diagrams: natural size,
--    capped to the text width)
--  * "In this chapter you will learn:" + bullet list -> #learn-box[...]
--  * heading identifiers get a per-chapter prefix (set via metadata
--    `idprefix`) so that labels are unique across the whole book
--  * `\&` in TeX math (the linear-logic "with") becomes a fullwidth ampersand,
--    which tools/pdf_build.py turns into Typst's `amp` symbol (a plain `&` is an
--    alignment point in Typst math)

local ALERTS = { note = true, tip = true, important = true, warning = true, caution = true }
local TEXT_WIDTH_PT = 440 -- A4 with the template margins, in points

local idprefix = ""

local function raw(s)
  return pandoc.RawBlock("typst", s)
end

local function typst_string(s)
  return '"' .. s:gsub("\\", "\\\\"):gsub('"', '\\"') .. '"'
end

local function svg_width_pt(path)
  local f = io.open(path, "r")
  if not f then
    return nil
  end
  local head = f:read(4096) or ""
  f:close()
  local w, unit = head:match('<svg[^>]-width="([%d%.]+)(%a*)"')
  if not w then
    return nil
  end
  w = tonumber(w)
  if unit == "px" or unit == "" then
    w = w * 0.75
  end
  return w
end

function Meta(meta)
  if meta.idprefix then
    idprefix = pandoc.utils.stringify(meta.idprefix)
  end
end

function Header(h)
  if h.identifier ~= "" then
    h.identifier = idprefix .. h.identifier
  end
  return h
end

function Div(div)
  local kind = div.classes[1]
  if not kind or not ALERTS[kind] then
    return nil
  end
  local content = pandoc.List()
  for _, b in ipairs(div.content) do
    -- Drop pandoc's generated title div ("Note", "Tip", ...).
    if not (b.t == "Div" and b.classes[1] == "title") then
      content:insert(b)
    end
  end
  local out = pandoc.List({ raw('#callout(kind: "' .. kind .. '")[') })
  out:extend(content)
  out:insert(raw("]"))
  return out
end

local function image_figure(img)
  local src = img.src
  local caption = pandoc.utils.stringify(img.caption)
  local width = nil
  if src:match("/screens/") or src:match("%.png$") or src:match("%.jpe?g$") then
    width = "92%"
  elseif src:match("%.svg$") then
    local w = svg_width_pt(src)
    if w and w > TEXT_WIDTH_PT then
      width = "100%"
    end
  end
  local img_call = "image(" .. typst_string(src)
  if width then
    img_call = img_call .. ", width: " .. width
  end
  img_call = img_call .. ")"
  -- Render the caption through pandoc so inline markup (code, emphasis) survives.
  local cap_typst = pandoc.write(pandoc.Pandoc({ pandoc.Plain(img.caption) }), "typst")
  cap_typst = cap_typst:gsub("%s+$", "")
  if caption == "" then
    return raw("#figure(" .. img_call .. ")")
  end
  return raw("#figure(" .. img_call .. ", caption: [" .. cap_typst .. "])")
end

local function is_learn_para(b)
  if b.t ~= "Para" or #b.content == 0 then
    return false
  end
  local first = b.content[1]
  if first.t ~= "Strong" then
    return false
  end
  local s = pandoc.utils.stringify(first):lower()
  return s:match("^in this chapter you will learn") ~= nil
    or s:match("^in this appendix you will") ~= nil
    or s:match("^what you will learn") ~= nil
end

function Blocks(blocks)
  local out = pandoc.List()
  local i = 1
  while i <= #blocks do
    local b = blocks[i]
    if b.t == "Para" and #b.content == 1 and b.content[1].t == "Image" then
      out:insert(image_figure(b.content[1]))
      i = i + 1
    elseif b.t == "Figure" then
      -- implicit_figures may already have produced a Figure: unwrap its image.
      local img = nil
      pandoc.walk_block(b, { Image = function(el) img = el end })
      if img then
        if #img.caption == 0 and b.caption and b.caption.long then
          img.caption = pandoc.utils.blocks_to_inlines(b.caption.long)
        end
        out:insert(image_figure(img))
      else
        out:insert(b)
      end
      i = i + 1
    elseif is_learn_para(b) and blocks[i + 1] and blocks[i + 1].t == "BulletList" then
      out:insert(raw("#learn-box["))
      out:insert(b)
      out:insert(blocks[i + 1])
      out:insert(raw("]"))
      i = i + 2
    else
      out:insert(b)
      i = i + 1
    end
  end
  return out
end

function Math(m)
  if m.text:find("\\&") then
    m.text = m.text:gsub("\\&", "\u{FF06}")
    return m
  end
  return nil
end

-- Typst has no grammar for these; they read well with the Rust one.
local HIGHLIGHT_AS = { ron = "rust" }

function CodeBlock(cb)
  local lang = cb.classes[1]
  if lang and HIGHLIGHT_AS[lang] then
    cb.classes[1] = HIGHLIGHT_AS[lang]
    return cb
  end
  return nil
end

-- Run Meta first so that the id prefix is known when headers are processed.
return {
  { Meta = Meta },
  { Div = Div, CodeBlock = CodeBlock, Math = Math },
  { Header = Header, Blocks = Blocks },
}
