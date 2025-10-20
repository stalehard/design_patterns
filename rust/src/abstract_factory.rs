pub trait Sedan {
    fn go_road(&self) -> &str;
}

pub trait SUV {
    fn go_road(&self) -> &str;
    fn go_offroad(&self) -> &str;
}

pub trait AutoFactory {
    type SedanType: Sedan;
    type SUVType: SUV;

    fn create_sedan(&self) -> Self::SedanType;
    fn create_suv(&self) -> Self::SUVType;
}

pub struct BMWSedan;
impl Sedan for BMWSedan {
    fn go_road(&self) -> &str {
        "Bmw sedan is going on the road"
    }
}

pub struct BMWSUV;
impl SUV for BMWSUV {
    fn go_road(&self) -> &str {
        "BMW SUV is going on the road"
    }

    fn go_offroad(&self) -> &str {
        "BMW SUV is going offroad"
    }
}

pub struct MercedesSedan;
impl Sedan for MercedesSedan {
    fn go_road(&self) -> &str {
        "Mercedes sedan is going on the road"
    }
}

pub struct MercedesSUV;
impl SUV for MercedesSUV {
    fn go_road(&self) -> &str {
        "Mercedes SUV is going on the road"
    }

    fn go_offroad(&self) -> &str {
        "Mercedes SUV is going offroad"
    }
}

pub struct BMWFactory;

impl AutoFactory for BMWFactory {
    type SedanType = BMWSedan;
    type SUVType = BMWSUV;
    fn create_sedan(&self) -> Self::SedanType {
        BMWSedan {}
    }

    fn create_suv(&self) -> Self::SUVType {
        BMWSUV {}
    }
}

pub struct MercedesFactory;

impl AutoFactory for MercedesFactory {
    type SedanType = MercedesSedan;
    type SUVType = MercedesSUV;
    fn create_sedan(&self) -> Self::SedanType {
        MercedesSedan {}
    }

    fn create_suv(&self) -> Self::SUVType {
        MercedesSUV {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mercedes_factory() {
        let factory = MercedesFactory;
        let sedan = factory.create_sedan();
        let suv = factory.create_suv();
        assert_eq!(sedan.go_road(), "Mercedes sedan is going on the road");
        assert_eq!(suv.go_road(), "Mercedes SUV is going on the road");
        assert_eq!(suv.go_offroad(), "Mercedes SUV is going offroad");
    }

    #[test]
    fn test_bmw_factory() {
        let factory = BMWFactory;
        let sedan = factory.create_sedan();
        let suv = factory.create_suv();
        assert_eq!(sedan.go_road(), "Bmw sedan is going on the road");
        assert_eq!(suv.go_road(), "BMW SUV is going on the road");
        assert_eq!(suv.go_offroad(), "BMW SUV is going offroad");
    }
}
