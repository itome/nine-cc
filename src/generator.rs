use crate::node::Node;

pub fn generate_program(program: &Vec<Node>) -> Vec<String> {
    let mut assembly: Vec<String> = vec!();
    assembly.push(".intel_syntax noprefix".to_string());
    assembly.push(".global main".to_string());
    assembly.push("main:".to_string());
    for stmt in program {
        assembly.append(&mut generate(&stmt));
    }
    assembly.push("  pop rax".to_string());
    assembly.push("  ret".to_string());
    return assembly;
}

fn generate(node: &Node) -> Vec<String> {
    let mut assembly: Vec<String> = vec!();
    if let Some(num) = node.number {
        assembly.push(format!("  push {}", num));
        return assembly;
    }
    if let Some(rhs) = &node.rhs {
        assembly.append(&mut generate(&rhs));
    }
    if let Some(lhs) = &node.lhs {
        assembly.append(&mut generate(&lhs));
    }
    assembly.push("  pop rax".to_string());
    assembly.push("  pop rdi".to_string());

    match &node.operator {
        Some(op) => match op.as_ref() {
            "+" => {
                assembly.push("  add rax, rdi".to_string());
            }
            "-" => {
                assembly.push("  sub rax, rdi".to_string());
            }
            "*" => {
                assembly.push("  imul rax, rdi".to_string());
            }
            "/" => {
                assembly.push("  cqo".to_string());
                assembly.push("  idiv rdi".to_string());
            }
            "==" => {
                assembly.push("  cmp rax, rdi".to_string());
                assembly.push("  sete al".to_string());
                assembly.push("  movzb rax, al".to_string());
            }
            "!=" => {
                assembly.push("  cmp rax, rdi".to_string());
                assembly.push("  setne al".to_string());
                assembly.push("  movzb rax, al".to_string());
            }
            "<" => {
                assembly.push("  cmp rax, rdi".to_string());
                assembly.push("  setl al".to_string());
                assembly.push("  movzb rax, al".to_string());
            }
            "<=" => {
                assembly.push("  cmp rax, rdi".to_string());
                assembly.push("  setle al".to_string());
                assembly.push("  movzb rax, al".to_string());
            }
            _ => {}
        },
        _ => {}
    }
    assembly.push("  push rax".to_string());
    return assembly;
}
