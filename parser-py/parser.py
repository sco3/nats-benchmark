
import re
import os

def parse_log_file(file_path):
    with open(file_path, 'r') as f:
        content = f.read()
        match = re.search(r'(\d{1,3}(?:,\d{3})*|\d+)\s+msgs/sec', content)
        if match:
            return match.group(1).replace(',', '')
    return None

def get_sort_key(dir_name):
    dir_name = dir_name.lower()
    if dir_name.endswith('ms'):
        return int(dir_name[:-2])
    elif dir_name.endswith('s'):
        return int(dir_name[:-1]) * 1000
    elif dir_name.endswith('m'):
        return int(dir_name[:-1]) * 60 * 1000
    else:
        return float('inf') # Should not happen with the given data

def main():
    data = {}
    for root, dirs, files in os.walk('.'):
        for file in files:
            if file.endswith('.log'):
                file_path = os.path.join(root, file)
                dir_name = os.path.basename(root)
                file_type = 'pub' if 'pub' in file else 'sub'
                msgs_sec = parse_log_file(file_path)
                if msgs_sec:
                    if dir_name not in data:
                        data[dir_name] = {}
                    data[dir_name][file_type] = msgs_sec

    # Sort data by directory name using the custom sort key
    sorted_dirs = sorted(data.keys(), key=get_sort_key)

    # Print Markdown table
    print('| Sync Period | Pub r/s | Sub r/s |')
    print('|---|---|---|')
    for dir_name in sorted_dirs:
        pub_rs = data[dir_name].get('pub', 'N/A')
        sub_rs = data[dir_name].get('sub', 'N/A')
        print(f"| {dir_name} | {pub_rs} | {sub_rs} |")

if __name__ == '__main__':
    main()
