Note
text - Just plain text
label - Label[]
title - text
comment - text
creation_date - UTC
updated_date - UTC
remind_me_about_it - UTC[]
author - id (string)
group - note group id

Label
text - string
important level int 0...10

User
id - (string)
avatar - img
notes - note ids string[]
group_notes - group notes []

// Methods
get_note_by_id GET
get_notes_by_user_id GET
get_group_notes_by_user_id GET
get_note_by_comment GET
get_note_by_title GET
get_note_by_value GET (only for text types)
update_note PUT
delete_note DELETE
delete_notes_by_group
