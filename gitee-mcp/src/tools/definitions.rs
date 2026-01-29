use crate::Tool;

pub fn get_tools_list() -> Vec<Tool> {
    let mut tools = Vec::new();
    
    tools.extend(crate::tools::issues::get_tool_definitions());
    tools.extend(crate::tools::pulls::get_tool_definitions());
    tools.extend(crate::tools::labels::get_tool_definitions());
    tools.extend(crate::tools::repos::get_tool_definitions());
    tools.extend(crate::tools::users::get_tool_definitions());
    tools.extend(crate::tools::notifications::get_tool_definitions());
    tools.extend(crate::tools::files::get_tool_definitions());
    tools.extend(crate::tools::wikis::get_tool_definitions());
    
    tools
}