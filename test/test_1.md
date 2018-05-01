Title: Markdown Requirements Management Tool
Type: Software Requirements Specification
Status: Draft

*History*

| Version | Date       | Author            | Change            |
|---------|------------|-------------------|-------------------|
| 1.0.0   | 2018-04-30 | jonas.wolf@gmx.eu | Initial version   | 


# Services
{MRQ-1 *MRQ shall provide a service to add a new requirement.*  
Adding means replacing a requirement identifier ending with `<new>` with the next free integer. 
If the word new is followed by an integer, all references to `<new>` with the same identifier will be replaced with the same requirement identifier. 

#type:functional  
}

{MRQ-2 *MRQ shall provide a service to check a requirement specificiation.*

The check shall include the following topics:
- Well-formedness
- Requirement duplicate
- Consistency within different specs
  - Config Mgmt sagt was zusammengehört
- Mandatory attributes set
  - Per Requirement
  - Per Document  
}

Remove
  Remove requirement definition
  Check requirement references
  Check history?
Impact
Create Diff Report
Create Report
  Including traceability
Renumerate
Enumerate
  Especially helpful for e.g. detailed design (first time use)
Upgrade config

# Use cases

- Attribute per Hashtag
- Requirement per Absatz
  - Wie über mehrere Absätze? Nicht notwendig, da sonst eh nicht atomar?
  - Oder closing } erst nach Ende des Absatz
- Config Versionierung? Änderungsmanagement, wenn sich Metamodell ändert?
  - Projektfile/Specfile verweist auf Config
- Caching basiernd auf Hash pro Projektfile

# Metamodel
- Metamodell
  - Projektfile
    - Projektfile #opt
    - Configfile #opt
    - Specfiles
      - Configfile #opt

# Other
If no config is given in Specfile (and nothing is inherited, i.e. the absolute default), the following applies:
 - The first identifier between `{` and `<space>` in the form of [A-Z]{2}[A-Z`\`-]+[(`<new>`|`\`d+)] is used for the prefix of the specification.  
  The configuration attribute would be ``Prefix:``
