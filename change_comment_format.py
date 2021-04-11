
import os
import re
from os.path import join
from glob import glob


files = []
for ext in ('*.h', '*.c'):
   files.extend(glob(join("./**/*", ext), recursive=True))

for fpath in files:
    print(fpath)
    changed = False
    changed_text = ''
    with open(fpath, 'r', encoding='utf8') as f:        
        text = f.read()
        regex = r'(/\*)(.*?)(\*/)'
        m = re.search(regex, text)
        if m is not None:
            #print("// " + m.group(2))
            changed_text = re.sub(regex, r'//\2' , text)
            changed = True
            print(changed_text)
    
    if changed:
        with open(fpath, 'w', encoding='utf8') as f:
            f.write(changed_text)

        # if len(m) > 0:
        #     changed = True
        
        # changed_text = re.sub(regex, r'// \2' , text)
        # print(m)
    
    # if changed:
    #     with open(f, 'w', encoding='utf8') as f:
    #         f.write(changed_text)

# files_grabbed is the list of pdf and cpp files


# for filename in files:
#     with open(filename, 'r', encoding='utf8') as f:
#         text = f.read()

#     # process Unicode text

#     with open(filename, 'w', encoding='utf8') as f:
#         f.write(text)
