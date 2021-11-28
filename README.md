# ORG Sync

## Entrypoints

On Save of org buffer:
1. Convert Org file text into AST (Indextree representation)
2. DFS over the AST and update database if there's any change in the data
3. If there's a change, update the last-updated timestamp in the table and reason as org
4. Pull changes from vdirsyncer
5. If any changes, run vdirsyncer entrypoint
6. Do conflict management if there are conflicting changes. Conflict resolution will happen through
   last-updated timestamp.

On vdirsyncer hook:
1. Parse ics file
2. Find task with UUID and update if necessary 
3. Check for last-updated timestamp conflict. If org wins, skip 
4. Update last-updated timestamp and reason as caldav
5. Confict resolution

## Conflict Resolution

On Org win conflict:
1. Convert this AST into the respective ical representation.
2. Overwrite the respective ics file
3. Trigger vdirsyncer to push the changes to the caldav server

On ical win conflict:
1. Convert JSON back to AST
2. Create ORG file from AST
3. Generate ORG text and overwrite the respective file
