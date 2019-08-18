use crate::node::Node;

pub fn gen_program(program: &Vec<Node>) -> Vec<String> {
    let mut assembly: Vec<String> = vec!();
    assembly.push(".intel_syntax noprefix".to_string());
    assembly.push(".global main".to_string());
    assembly.push("main:".to_string());
    assembly.push("  push rbp".to_string());
    assembly.push("  mov rbp, rsp".to_string());
    assembly.push("  sub rsp, 208".to_string());
    for stmt in program {
        let (generated, returned) = &mut gen(&stmt);
        assembly.append(generated);
        assembly.push("  pop rax".to_string());
        if *returned {
            break;
        }
    }
    assembly.push("  mov rsp, rbp".to_string());
    assembly.push("  pop rbp".to_string());
    assembly.push("  ret".to_string());
    return assembly;
}

fn gen_lval(node: &Node) -> Vec<String> {
    let mut assembly: Vec<String> = vec!();
    match node.offset {
        Some(offset) => {
            assembly.push("  mov rax, rbp".to_string());
            assembly.push(format!("  sub rax, {}", offset));
            assembly.push("  push rax".to_string());
        }
        _ => {
            panic!("The lvalue of the assignment is not a variable")
        }
    }
    return assembly;
}

fn gen(node: &Node) -> (Vec<String>, bool) {
    let mut assembly: Vec<String> = vec!();
    if let Some(num) = node.number {
        assembly.push(format!("  push {}", num));
        return (assembly, false);
    }
    if let Some(_) = node.offset {
        assembly.append(&mut gen_lval(node));
        assembly.push("  pop rax".to_string());
        assembly.push("  mov rax, [rax]".to_string());
        assembly.push("  push rax".to_string());
        return (assembly, false);
    }
    if node.operator == Some("return".to_string()) {
        if let Some(lhs) = &node.lhs {
            let (generated, _) = &mut gen(&lhs);
            assembly.append(generated);
        }
        return (assembly, true);
    }
    if node.operator == Some("=".to_string()) {
        if let Some(lhs) = &node.lhs {
            assembly.append(&mut gen_lval(&lhs));
        }
        if let Some(rhs) = &node.rhs {
            let (generated, _) = &mut gen(&rhs);
            assembly.append(generated);
        }
        assembly.push("  pop rdi".to_string());
        assembly.push("  pop rax".to_string());
        assembly.push("  mov [rax], rdi".to_string());
        assembly.push("  push rdi".to_string());
        return (assembly, false);
    }
    if let Some(rhs) = &node.rhs {
        let (generated, _) = &mut gen(&rhs);
        assembly.append(generated);
    }
    if let Some(lhs) = &node.lhs {
        let (generated, _) = &mut gen(&lhs);
        assembly.append(generated);
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
    return (assembly, false);
}
