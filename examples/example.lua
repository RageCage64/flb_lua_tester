
function filter_entry(tag, timestamp, record)
    record["y"] = "z"
    return 0, timestamp, record
end