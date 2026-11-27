# Regenerates the launcher icons: python3 art/gen.py  (needs rsvg-convert)
import os, subprocess

floor = '<rect width="108" height="108" fill="#8ecae6"/><rect y="68" width="108" height="40" fill="#d9a066"/>' \
  '<g stroke="#a9713f" stroke-width="1" fill="none"><path d="M0 68H108M0 82H108M0 96H108M30 68V82M78 68V82M52 82V96M96 82V96M24 96V108M72 96V108"/></g>'

ticks = ''.join(f'<path d="M{x} 76v{5 if i%2==0 else 3}"/>' for i,x in enumerate(range(28,80,4)))

# Original art: a rabbit crouching, pulling a measuring tape along the floor.
rabbit = f'''
<g stroke="#4a3b35" stroke-width="1.4" stroke-linejoin="round">
 <circle cx="26" cy="61" r="6" fill="#fff"/>
 <ellipse cx="42" cy="62" rx="18" ry="13" fill="#f4f1ee"/>
 <ellipse cx="36" cy="66" rx="11" ry="9" fill="#f4f1ee"/>
 <ellipse cx="41" cy="74" rx="12" ry="3.8" fill="#fff"/>
 <g transform="rotate(-20 55 38)"><ellipse cx="55" cy="30" rx="5" ry="16" fill="#f4f1ee"/><ellipse cx="55" cy="30" rx="2.2" ry="11.5" fill="#f4a6b0" stroke="none"/></g>
 <g transform="rotate(8 66 36)"><ellipse cx="66" cy="29" rx="5" ry="16" fill="#f4f1ee"/><ellipse cx="66" cy="29" rx="2.2" ry="11.5" fill="#f4a6b0" stroke="none"/></g>
 <circle cx="63" cy="52" r="14" fill="#f4f1ee"/>
</g>
<circle cx="68" cy="50" r="2.4" fill="#2b2220"/><circle cx="68.8" cy="49.2" r=".8" fill="#fff"/>
<ellipse cx="76" cy="56" rx="2.2" ry="1.6" fill="#e5707e"/>
<circle cx="67" cy="58" r="2.8" fill="#f4a6b0" opacity=".6"/>
<path d="M75.500 58.500q-2 3-5 1.500M73 54l10-3M73 58l10 1" stroke="#4a3b35" stroke-width=".9" fill="none" stroke-linecap="round"/>
<g stroke="#4a3b35" stroke-width="1.2">
 <rect x="25" y="76" width="55" height="9" rx="1" fill="#ffd23f"/>
 <rect x="77" y="72" width="11" height="14" rx="3" fill="#f28c28"/>
 <circle cx="82.500" cy="79" r="2.5" fill="#ddd"/>
 <path d="M55 60Q69 64 72 75" stroke-width="7.5" stroke-linecap="round" fill="none"/>
 <path d="M55 60Q69 64 72 75" stroke="#f4f1ee" stroke-width="5" stroke-linecap="round" fill="none"/>
</g>
<g stroke="#4a3b35" stroke-width=".9">{ticks}</g>'''

def svg(body, view="0 0 108 108"):
    return f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="{view}">{body}</svg>'

# Adaptive layers: the foreground stays inside the central safe zone.
layers = {
  'ic_background': svg(floor),
  'ic_foreground': svg(f'<g transform="translate(54 54) scale(.9) translate(-55 -52)">{rabbit}</g>'),
  'ic_launcher': svg('<defs><clipPath id="c"><rect x="14" y="14" width="80" height="80" rx="18"/></clipPath></defs>'
                     f'<g clip-path="url(#c)">{floor}<g transform="translate(54 55) scale(.93) translate(-55 -52)">{rabbit}</g></g>', "14 14 80 80"),
}
for name, data in layers.items():
    open(f'art/{name}.svg','w').write(data)
    for d, s in dict(mdpi=1, hdpi=1.5, xhdpi=2, xxhdpi=3, xxxhdpi=4).items():
        os.makedirs(f'res/mipmap-{d}', exist_ok=True)
        px = int((48 if name=='ic_launcher' else 108)*s)
        subprocess.run(['rsvg-convert','-w',str(px),'-h',str(px),f'art/{name}.svg','-o',f'res/mipmap-{d}/{name}.png'],check=True)
os.makedirs('res/mipmap-anydpi-v26', exist_ok=True)
open('res/mipmap-anydpi-v26/ic_launcher.xml','w').write('''<?xml version="1.0" encoding="utf-8"?>
<adaptive-icon xmlns:android="http://schemas.android.com/apk/res/android">
    <background android:drawable="@mipmap/ic_background"/>
    <foreground android:drawable="@mipmap/ic_foreground"/>
</adaptive-icon>
''')
subprocess.run(['rsvg-convert','-w','512','-h','512','art/ic_launcher.svg','-o','art/icon-512.png'],check=True)
