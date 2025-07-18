fn vos_md5_calc_digest_of_buff(context: &mut MD5_CTX) {
    let mut i: u32;
    let mut tmp_value: u32;
    let mut text_fragment = [0u32; 16];
    let mut tmp_state = [0u32; 4];
    let mut tmp_text = &context.auc_buffer[..];
    
    tmp_state[0] = context.aul_state[0];
    tmp_state[1] = context.aul_state[1];
    tmp_state[2] = context.aul_state[2];
    tmp_state[3] = context.aul_state[3];
    
    for i in 0..16 {
        if i % 4 == 0 {
            text_fragment[i] = u32::from(tmp_text[0]) 
                | (u32::from(tmp_text[1]) << 8) 
                | (u32::from(tmp_text[2]) << 16) 
                | (u32::from(tmp_text[3]) << 24;
            text_fragment[i + 1] = u32::from(tmp_text[4]) 
                | (u32::from(tmp_text[5]) << 8 
                | (u32::from(tmp_text[6]) << 16) 
                | (u32::from(tmp_text[7]) << 24;
            text_fragment[i + 2] = u32::from(tmp_text[8]) 
                | (u32::from(tmp_text[9]) << 8 
                | (u32::from(tmp_text[10]) << 16) 
                | (u32::from(tmp_text[11]) << 24;
            text_fragment[i + 3] = u32::from(tmp_text[12]) 
                | (u32::from(tmp_text[13]) << 8) 
                | (u32::from(tmp_text[14]) << 16) 
                | (u32::from(tmp_text[15]) << 24;
            tmp_text = &tmp_text[16..];
        }
    }
    
    md5_f_proc(&mut tmp_value, &mut tmp_state, &text_fragment);
    md5_g_proc(&mut tmp_value, &mut tmp_state, &text_fragment);
    md5_h_proc(&mut tmp_value, &mut tmp_state, &text_fragment);
    md5_i_proc(&mut tmp_value, &mut tmp_state, &text_fragment);
    
    context.aul_state[0] += tmp_state[0];
    context.aul_state[1] += tmp_state[1];
    context.aul_state[2] += tmp_state[2];
    context.aul_state[3] += tmp_state[3];
}
