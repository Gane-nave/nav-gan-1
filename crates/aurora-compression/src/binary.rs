// Assuming some context of the file, make sure to locate the right section before the fix

// Original code:
// Vec::with_capacity(count);

// Update to:
Vec::with_capacity(count.min((data.len() - *offset) / 1));

// Same logic for MAP:
// Vec::with_capacity(count);
Vec::with_capacity(count.min((data.len() - *offset) / 1));
