pub fn vos_md5_final_ex(digest: &mut [u8], buf_len: usize, context: &mut MD5_CTX) {
    let mut need_another_buff = false;

    if digest.is_empty() || context.is_null() || buf_len < MD5_DIGEST_LEN {
        return;
    }

    need_another_buff = vos_md5_pad_buff(context);
    vos_md5_calc_digest_of_buff(context);

    if need_another_buff {
        context.ui_pos = 0;
        while context.ui_pos < MD5_TEXT_IN_BUFFER_MAX {
            context.auc_buffer[context.ui_pos] = 0;
            context.ui_pos += 1;
        }
        md5_record_message_len(context);
        vos_md5_calc_digest_of_buff(context);
    }

    md5_compose_digest(digest, &context.aul_state);

    // Zero out the context for security
    context.auc_buffer.iter_mut().for_each(|x| *x = 0);
    context.aul_state.iter_mut().for_each(|x| *x = 0);
    context.ui_pos = 0;
    context.ui_total_len = 0;
}
