# How to add a new tool to the list

Please feel free to open a pull request if you know of a code analysis tool that
is not mentioned here.  
If you're in doubt if a tool is a good fit for the list, **don't open an issue,
but create a pull request right away** because that's easier to handle. Thanks!
:smiley:

### Requirements

Each tool on the list should be

- actively maintained
- actively used (have more than ten Stars on Github or similar impact)
- relatively mature (project exists for at least one month)

### Format

The main `README.md` is just a rendered version of the data. To add a new tool,
please add it to `data/data.yml`.

- Make each tool description as precise as possible.  
  Please limit the description to **200 characters**.
- The order of the tools is not relevant.
- Please add as many categories as possible.

Once you're done, call `make lint` to check for errors.

# How to mark a tool as unmaintained/deprecated

Sometimes a tool becomes unmaintained and there's nothing wrong with that.  
After all, a tool can still be very valuable to the community - even without
frequent updates.  
However, since it is one of the goals of this project to allow people to make an
informed decision on what is the best tool for the job, we are marking
unmaintained or deprecated tools. To do so, add `deprecated: true` to the entry
in `data/data.yml`.

[Here](https://github.com/mre/awesome-static-analysis/issues/223) is a nice
discussion about why we think this is necessary. If you find a tool, which is
unmaintained, please create a pull request and provide an objective explanation
as to why you think the tool should be marked. Every deprecation will be handled
on a case-by-case basis.

**Thanks for helping out!** :tada:
