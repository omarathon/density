use crate::common::prepare_default_test_file;
use divan;

mod common;

fn main() {
    prepare_default_test_file();
    divan::main();
}

#[divan::bench_group(sample_count = 25)]
mod chameleon {
    use crate::common::test_file_path;
    use density::algorithms::chameleon::chameleon::Chameleon;
    use divan::counter::BytesCount;
    use divan::Bencher;
    use std::fs::read;

    #[divan::bench(name = "compress/raw")]
    fn encode_raw(bencher: Bencher) {
        let in_mem = read(test_file_path()).unwrap();
        let mut out_mem = vec![0_u8; in_mem.len() << 1];

        print!("\r\t\t\t({:.3}x)   ", in_mem.len() as f64 / Chameleon::encode(&in_mem, &mut out_mem).unwrap() as f64);

        bencher
            .counter(BytesCount::of_slice(&in_mem))
            .bench_local(|| { Chameleon::encode(&in_mem, &mut out_mem) });
    }

    #[divan::bench(name = "decompress/raw             ")]
    fn decode_raw(bencher: Bencher) {
        let in_mem = read(test_file_path()).unwrap();
        let mut out_mem = vec![0_u8; in_mem.len() << 1];
        let size = Chameleon::encode(&in_mem, &mut out_mem).unwrap();
        let mut dec_mem = vec![0_u8; in_mem.len() << 1];

        let d_size = Chameleon::decode(&out_mem[0..size], &mut dec_mem).unwrap();
        assert_eq!(in_mem.len(), d_size);
        for i in 0..d_size {
            // if in_mem[i] != dec_mem[i] {
            //     dbg!(&i);
            // }
            assert_eq!(in_mem[i], dec_mem[i]);
        }

        print!("\r\t\t\t({})  ", d_size);

        bencher
            .counter(BytesCount::of_slice(&in_mem))
            .bench_local(|| { Chameleon::decode(&out_mem[0..size], &mut dec_mem) });
    }
    //
    // #[divan::bench(name = "roundtrip/raw")]
    // fn roundtrip_raw(bencher: Bencher) {
    //     let in_mem = read(&format!("{}{}", DATA_DIRECTORY, TEST_FILE)).unwrap();
    //     let mut out_mem = vec![0_u8; in_mem.len() << 1];
    //     let mut dec_mem = vec![0_u8; in_mem.len() << 1];
    //
    //     bencher
    //         .counter(BytesCount::new(in_mem.len() * 2))
    //         .bench_local(|| {
    //             let size = Chameleon::encode(&in_mem, &mut out_mem).unwrap();
    //             Chameleon::decode(&out_mem[0..size], &mut dec_mem)
    //         });
    // }
}

#[divan::bench_group(sample_count = 25)]
mod cheetah {
    use crate::common::test_file_path;
    use density::algorithms::cheetah::cheetah::Cheetah;
    use divan::counter::BytesCount;
    use divan::Bencher;
    use std::fs::read;

    #[divan::bench(name = "compress/raw")]
    fn encode_raw(bencher: Bencher) {
        let in_mem = read(test_file_path()).unwrap();
        let mut out_mem = vec![0_u8; in_mem.len() << 1];

        print!("\r\t\t\t({:.3}x)   ", in_mem.len() as f64 / Cheetah::encode(&in_mem, &mut out_mem).unwrap() as f64);

        bencher
            .counter(BytesCount::of_slice(&in_mem))
            .bench_local(|| { Cheetah::encode(&in_mem, &mut out_mem) });
    }

    #[divan::bench(name = "decompress/raw")]
    fn decode_raw(bencher: Bencher) {
        let in_mem = read(test_file_path()).unwrap();
        let mut out_mem = vec![0_u8; in_mem.len() << 1];
        let size = Cheetah::encode(&in_mem, &mut out_mem).unwrap();
        let mut dec_mem = vec![0_u8; in_mem.len() << 1];

        let d_size = Cheetah::decode(&out_mem[0..size], &mut dec_mem).unwrap();
        assert_eq!(in_mem.len(), d_size);
        for i in 0..d_size {
            // if in_mem[i] != dec_mem[i] {
            //     dbg!(&i);
            // }
            assert_eq!(in_mem[i], dec_mem[i]);
        }

        print!("\r\t\t\t({})  ", d_size);

        bencher
            .counter(BytesCount::of_slice(&in_mem))
            .bench_local(|| { Cheetah::decode(&out_mem[0..size], &mut dec_mem) });
    }
}