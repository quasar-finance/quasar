import json
import subprocess
import re


def run_command(command):
    process = subprocess.Popen(
        command, shell=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    output, error = process.communicate()
    return output.decode('utf-8'), error.decode('utf-8')


prim1 = "quasar1kj8q8g2pmhnagmfepp9jh9g2mda7gzd0m5zdq0s08ulvac8ck4dq9ykfps"
prim2 = "quasar1ma0g752dl0yujasnfs9yrk6uew7d0a2zrgvg62cfnlfftu2y0egqx8e7fv"
prim3 = "quasar1ery8l6jquynn9a4cz2pff6khg8c68f7urt33l5n9dng2cwzz4c4qxhm6a2"
node = "--node https://quasar-rpc.polkachu.com:443 --chain-id quasar-1 -o json"
query = '\'{\"trapped_errors\": {}}\''

# Create a dictionary to store bond_id lists
bond_ids = {"prim1": [], "prim2": [], "prim3": []}

for i, prim in enumerate([prim1, prim2, prim3]):
    output, error = run_command(
        f'quasard q wasm contract-state smart {prim} {query} {node} | tee {prim[-3:]}_trapped_errors.json')

    data = json.loads(output) if not error else print(
        'Error executing command 1:', error)

    # Get the errors from the data
    errors = data["data"]["errors"]

    # Iterate over the errors
    for key, value in errors.items():
        # Check if key matches the desired format
        if re.match(r'\d+-channel-\d+', key):
            # If the error includes 'step' data
            if 'step' in value and isinstance(value['step'], dict):
                steps = value['step']

                # Check if steps includes 'ica' data
                if 'ica' in steps:
                    ica = steps['ica']

                    # Check if ica includes 'join_swap_extern_amount_in' data
                    if 'join_swap_extern_amount_in' in ica:
                        join_swap = ica['join_swap_extern_amount_in']

                        # Check if join_swap includes 'bonds' data
                        if 'bonds' in join_swap:
                            bonds = join_swap['bonds']

                            # Iterate over the bonds
                            for bond in bonds:
                                # If bond includes 'bond_id'
                                if 'bond_id' in bond:
                                    bond_id = bond['bond_id']

                                    # Add the tuple (key, bond_id) to the appropriate list
                                    bond_ids[f"prim{i+1}"].append(
                                        (key, bond_id))

# save all bond_ids, per primitive, to a json file
with open('bond_ids.json', 'w') as f:
    json.dump(bond_ids, f)

vault = "quasar18a2u6az6dzw528rptepfg6n49ak6hdzkf8ewf0n5r0nwju7gtdgqamr7qu"

# get bond_ids that are null for prim1
null_ids_prim1 = []
for id in bond_ids['prim1']:
    output, error = run_command(
        f'quasard q wasm contract-state smart {vault} \'{{"pending_bonds_by_id": {{"bond_id": "{id[1]}"}}}}\' {node}')
    if ':null' in output:
        null_ids_prim1.append(id)

# get bond_ids that are null for prim2
null_ids_prim2 = []
for id in bond_ids['prim2']:
    output, error = run_command(
        f'quasard q wasm contract-state smart {vault} \'{{"pending_bonds_by_id": {{"bond_id": "{id[1]}"}}}}\' {node}')
    if ':null' in output:
        null_ids_prim2.append(id)

# get bond_ids that are null for prim3
null_ids_prim3 = []
for id in bond_ids['prim3']:
    output, error = run_command(
        f'quasard q wasm contract-state smart {vault} \'{{"pending_bonds_by_id": {{"bond_id": "{id[1]}"}}}}\' {node}')
    if ':null' in output:
        null_ids_prim3.append(id)

print(f'PRIM1: {null_ids_prim1}')
print(f'PRIM2: {null_ids_prim2}')
print(f'PRIM3: {null_ids_prim3}')

# Filter bond_ids for each primitive by checking if each tuple is not in the corresponding null_ids list

filtered_bond_ids_prim1 = [
    id for id in bond_ids['prim1'] if id not in null_ids_prim1]
filtered_bond_ids_prim2 = [
    id for id in bond_ids['prim2'] if id not in null_ids_prim2]
filtered_bond_ids_prim3 = [
    id for id in bond_ids['prim3'] if id not in null_ids_prim3]

# Print filtered bond_ids
print("Filtered bond_ids for prim1:", filtered_bond_ids_prim1)
print("Filtered bond_ids for prim2:", filtered_bond_ids_prim2)
print("Filtered bond_ids for prim3:", filtered_bond_ids_prim3)

print("Number of filtered bond_ids for prim1:", len(filtered_bond_ids_prim1))
print("Number of unfiltered bond_ids for prim1:", len(bond_ids['prim1']))
print("Number of filtered bond_ids for prim2:", len(filtered_bond_ids_prim2))
print("Number of unfiltered bond_ids for prim2:", len(bond_ids['prim2']))
print("Number of filtered bond_ids for prim3:", len(filtered_bond_ids_prim3))
print("Number of unfiltered bond_ids for prim3:", len(bond_ids['prim3']))
