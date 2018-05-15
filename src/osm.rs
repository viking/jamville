#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "osm")]
pub struct OSM {
    pub version: String,
    pub generator: String,
    pub note: String,
    pub meta: Meta,
    pub bounds: Bounds,
    #[serde(rename = "node", default)]
    pub nodes: Vec<Node>,
    #[serde(rename = "way", default)]
    pub ways: Vec<Way>,
    #[serde(rename = "relation", default)]
    pub relations: Vec<Relation>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "meta")]
pub struct Meta {
    pub osm_base: String
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "bounds")]
pub struct Bounds {
    pub minlat: f64,
    pub minlon: f64,
    pub maxlat: f64,
    pub maxlon: f64
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "node")]
pub struct Node {
    pub id: i64,
    pub lat: f64,
    pub lon: f64,
    pub version: u16,
    pub timestamp: String,
    pub changeset: u64,
    pub uid: Option<i64>,
    pub user: Option<String>,
    #[serde(rename = "tag", default)]
    pub tags: Vec<Tag>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "tag")]
pub struct Tag {
    pub k: String,
    pub v: String
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "way")]
pub struct Way {
    pub id: i64,
    pub version: u16,
    pub timestamp: String,
    pub changeset: u64,
    pub uid: Option<i64>,
    pub user: Option<String>,
    #[serde(rename = "nd", default)]
    pub node_refs: Vec<NodeRef>,
    #[serde(rename = "tag", default)]
    pub tags: Vec<Tag>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "nd")]
pub struct NodeRef {
    #[serde(rename = "ref")]
    pub id: i64
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "relation")]
pub struct Relation {
    pub id: i64,
    pub version: u16,
    pub timestamp: String,
    pub changeset: u64,
    pub uid: Option<i64>,
    pub user: Option<String>,
    #[serde(rename = "member", default)]
    pub members: Vec<Member>,
    #[serde(rename = "tag", default)]
    pub tags: Vec<Tag>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "member")]
pub struct Member {
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(rename = "ref")]
    pub id: u64,
    pub role: String
}

#[test]
fn it_works() {
    let s = r##"
        <?xml version="1.0" encoding="UTF-8"?>
        <osm version="0.6" generator="Overpass API 0.7.55 579b1eec">
            <note>The data included in this document is from www.openstreetmap.org. The data is made available under ODbL.</note>
            <meta osm_base="2018-05-14T21:45:02Z"/>
            <bounds minlat="35.9717000" minlon="-87.0639000" maxlat="36.3594000" maxlon="-86.4627000"/>
            <node id="37060116" lat="36.0872268" lon="-87.0759046" version="30" timestamp="2012-07-18T16:22:48Z" changeset="12288724" uid="722137" user="OSMF Redaction Account"/>
            <node id="37060125" lat="36.0873400" lon="-87.0759682" version="1" timestamp="2007-08-30T23:16:11Z" changeset="277489" uid="10759" user="WJHildreth">
              <tag k="created_by" v="JOSM"/>
            </node>
            <way id="5258226" version="5" timestamp="2011-07-28T05:24:26Z" changeset="8850994" uid="207745" user="NE2">
                <nd ref="37095773"/>
                <nd ref="37095766"/>
                <nd ref="37095758"/>
                <nd ref="472461326"/>
                <nd ref="37095750"/>
                <tag k="bridge" v="yes"/>
                <tag k="highway" v="motorway_link"/>
                <tag k="lanes" v="2"/>
                <tag k="layer" v="1"/>
                <tag k="oneway" v="yes"/>
            </way>
            <relation id="23148" version="180" timestamp="2018-05-03T20:43:54Z" changeset="58660134" uid="8214747" user="Pjones">
                <member type="way" ref="19594628" role="forward"/>
                <member type="way" ref="51640814" role=""/>
                <member type="way" ref="111039491" role=""/>
                <member type="way" ref="321391416" role=""/>
                <member type="way" ref="120254905" role=""/>
                <member type="way" ref="19594672" role=""/>
                <member type="way" ref="321375804" role=""/>
                <member type="way" ref="179809309" role=""/>
                <member type="way" ref="321391420" role=""/>
                <member type="way" ref="321391418" role=""/>
                <member type="way" ref="321393033" role=""/>
                <member type="way" ref="19598718" role=""/>
                <member type="way" ref="120254969" role=""/>
                <member type="way" ref="95093653" role=""/>
                <member type="way" ref="120254970" role=""/>
                <member type="way" ref="19598116" role=""/>
                <member type="way" ref="19591306" role=""/>
                <member type="way" ref="19597258" role=""/>
                <member type="way" ref="19531818" role=""/>
                <member type="way" ref="357071993" role=""/>
                <member type="way" ref="19535893" role=""/>
                <member type="way" ref="19531371" role=""/>
                <member type="way" ref="357072041" role=""/>
                <member type="way" ref="19531375" role=""/>
                <member type="way" ref="19530586" role=""/>
                <member type="way" ref="19553830" role=""/>
                <member type="way" ref="132956159" role=""/>
                <member type="way" ref="288398197" role=""/>
                <member type="way" ref="120254851" role=""/>
                <member type="way" ref="120254870" role=""/>
                <member type="way" ref="288398196" role=""/>
                <member type="way" ref="201167835" role=""/>
                <member type="way" ref="201167832" role=""/>
                <member type="way" ref="107445154" role=""/>
                <member type="way" ref="107445152" role=""/>
                <member type="way" ref="201167827" role=""/>
                <member type="way" ref="201167840" role=""/>
                <member type="way" ref="19546099" role=""/>
                <member type="way" ref="51640776" role=""/>
                <member type="way" ref="120254780" role=""/>
                <member type="way" ref="255172920" role=""/>
                <member type="way" ref="255172911" role=""/>
                <member type="way" ref="110675930" role="forward"/>
                <member type="way" ref="255172918" role="forward"/>
                <member type="way" ref="255172921" role="forward"/>
                <member type="way" ref="98047165" role="forward"/>
                <member type="way" ref="98047161" role="forward"/>
                <member type="way" ref="110675933" role="forward"/>
                <member type="way" ref="98047234" role="forward"/>
                <member type="way" ref="98047237" role="forward"/>
                <member type="way" ref="98047173" role="forward"/>
                <member type="way" ref="19553593" role="forward"/>
                <member type="way" ref="19548906" role="forward"/>
                <member type="way" ref="19546022" role="forward"/>
                <member type="way" ref="51640784" role="forward"/>
                <member type="way" ref="255172915" role="forward"/>
                <member type="way" ref="255172919" role="forward"/>
                <member type="way" ref="255172917" role="forward"/>
                <member type="way" ref="255172922" role="forward"/>
                <member type="way" ref="98047164" role="forward"/>
                <member type="way" ref="98047196" role="forward"/>
                <member type="way" ref="98047200" role="forward"/>
                <member type="way" ref="98047239" role="forward"/>
                <member type="way" ref="98047172" role="forward"/>
                <member type="way" ref="19553595" role="forward"/>
                <member type="way" ref="19546309" role="forward"/>
                <member type="way" ref="19550862" role="forward"/>
                <member type="way" ref="52134606" role="forward"/>
                <member type="way" ref="51640779" role=""/>
                <member type="way" ref="240736479" role=""/>
                <member type="way" ref="240736478" role=""/>
                <member type="way" ref="259818684" role=""/>
                <member type="way" ref="192543964" role=""/>
                <member type="way" ref="19552114" role=""/>
                <member type="way" ref="19546435" role=""/>
                <member type="way" ref="259818685" role=""/>
                <member type="way" ref="259818681" role=""/>
                <member type="way" ref="51652377" role=""/>
                <member type="way" ref="19423073" role=""/>
                <member type="way" ref="259774302" role=""/>
                <member type="way" ref="120254914" role=""/>
                <member type="way" ref="259774300" role=""/>
                <member type="way" ref="259774301" role=""/>
                <member type="way" ref="110675928" role="forward"/>
                <member type="way" ref="120254916" role="forward"/>
                <member type="way" ref="98557256" role="forward"/>
                <member type="way" ref="497053187" role="forward"/>
                <member type="way" ref="497053186" role="forward"/>
                <member type="way" ref="111000843" role="forward"/>
                <member type="way" ref="98557235" role="forward"/>
                <member type="way" ref="19417740" role="forward"/>
                <member type="way" ref="120254915" role="forward"/>
                <member type="way" ref="111000868" role="forward"/>
                <member type="way" ref="111000795" role="forward"/>
                <member type="way" ref="110675932" role="forward"/>
                <member type="way" ref="497053189" role="forward"/>
                <member type="way" ref="497053188" role="forward"/>
                <member type="way" ref="110675931" role="forward"/>
                <member type="way" ref="98557257" role="forward"/>
                <member type="way" ref="51652385" role="forward"/>
                <member type="way" ref="51652389" role=""/>
                <member type="way" ref="120254816" role=""/>
                <member type="way" ref="120254815" role=""/>
                <member type="way" ref="19424319" role=""/>
                <member type="way" ref="316157867" role=""/>
                <member type="way" ref="316157868" role=""/>
                <member type="way" ref="19419819" role=""/>
                <member type="way" ref="120254791" role=""/>
                <member type="way" ref="120254792" role=""/>
                <member type="way" ref="111458641" role=""/>
                <member type="way" ref="111458642" role=""/>
                <member type="way" ref="120254793" role=""/>
                <member type="way" ref="19424626" role=""/>
                <member type="way" ref="528512719" role=""/>
                <member type="way" ref="528512717" role=""/>
                <member type="way" ref="19392794" role=""/>
                <member type="way" ref="388355757" role=""/>
                <member type="way" ref="388355753" role=""/>
                <member type="way" ref="388355755" role=""/>
                <member type="way" ref="388355746" role=""/>
                <member type="way" ref="481438799" role=""/>
                <member type="way" ref="481438800" role=""/>
                <member type="way" ref="5133970" role=""/>
                <member type="way" ref="19389843" role=""/>
                <member type="way" ref="120254951" role=""/>
                <member type="way" ref="89762791" role=""/>
                <member type="way" ref="6834784" role=""/>
                <member type="way" ref="6832199" role=""/>
                <member type="way" ref="6834798" role=""/>
                <member type="way" ref="110675925" role=""/>
                <member type="way" ref="193796977" role=""/>
                <member type="way" ref="6837765" role=""/>
                <member type="way" ref="4994155" role=""/>
                <member type="way" ref="189060986" role=""/>
                <member type="way" ref="4968433" role=""/>
                <member type="way" ref="193796980" role=""/>
                <member type="way" ref="259818121" role=""/>
                <member type="way" ref="110675938" role=""/>
                <member type="way" ref="120254952" role=""/>
                <member type="way" ref="120254950" role=""/>
                <member type="way" ref="6841350" role=""/>
                <member type="way" ref="4973882" role=""/>
                <member type="way" ref="4973909" role=""/>
                <member type="way" ref="4973984" role=""/>
                <member type="way" ref="286612896" role=""/>
                <member type="way" ref="286612899" role=""/>
                <member type="way" ref="286612898" role=""/>
                <member type="way" ref="286612897" role=""/>
                <member type="way" ref="6771957" role=""/>
                <member type="way" ref="120254966" role=""/>
                <member type="way" ref="303428542" role="west"/>
                <member type="way" ref="303428543" role="west"/>
                <member type="way" ref="111081825" role="west"/>
                <member type="way" ref="110675926" role=""/>
                <member type="way" ref="111081777" role=""/>
                <member type="way" ref="365370765" role=""/>
                <member type="way" ref="365370764" role=""/>
                <member type="way" ref="365370763" role=""/>
                <member type="way" ref="306683294" role=""/>
                <member type="way" ref="306683295" role=""/>
                <member type="way" ref="110675927" role=""/>
                <member type="way" ref="6771965" role=""/>
                <member type="way" ref="120254797" role=""/>
                <member type="way" ref="120254798" role=""/>
                <member type="way" ref="6770522" role=""/>
                <member type="way" ref="6774356" role=""/>
                <member type="way" ref="111081839" role=""/>
                <member type="way" ref="111081842" role=""/>
                <member type="way" ref="6770554" role=""/>
                <member type="way" ref="6774350" role=""/>
                <member type="way" ref="6770551" role=""/>
                <member type="way" ref="19421772" role=""/>
                <member type="way" ref="120254948" role=""/>
                <member type="way" ref="120254949" role=""/>
                <member type="way" ref="19421783" role=""/>
                <member type="way" ref="111559712" role=""/>
                <member type="way" ref="111559758" role=""/>
                <member type="way" ref="19456495" role=""/>
                <member type="way" ref="108045569" role=""/>
                <member type="way" ref="19451023" role="forward"/>
                <member type="way" ref="108045392" role="forward"/>
                <member type="way" ref="120254801" role="forward"/>
                <member type="way" ref="108045545" role="forward"/>
                <member type="way" ref="120254802" role=""/>
                <member type="way" ref="19483391" role=""/>
                <member type="way" ref="120254800" role=""/>
                <member type="way" ref="539162964" role=""/>
                <member type="way" ref="539162961" role=""/>
                <member type="way" ref="539162963" role=""/>
                <member type="way" ref="108045631" role=""/>
                <member type="way" ref="108045493" role=""/>
                <member type="way" ref="245915228" role=""/>
                <member type="way" ref="245915229" role=""/>
                <member type="way" ref="340808717" role=""/>
                <member type="way" ref="340808718" role=""/>
                <member type="way" ref="495239401" role=""/>
                <member type="way" ref="51640792" role=""/>
                <member type="way" ref="19459539" role="forward"/>
                <member type="way" ref="48729807" role="forward"/>
                <member type="way" ref="27877543" role="forward"/>
                <member type="way" ref="51640798" role="forward"/>
                <member type="way" ref="48729798" role="forward"/>
                <member type="way" ref="19462660" role="forward"/>
                <member type="way" ref="223687991" role=""/>
                <member type="way" ref="223687992" role=""/>
                <member type="way" ref="19482379" role=""/>
                <member type="way" ref="120254796" role=""/>
                <member type="way" ref="120254795" role=""/>
                <member type="way" ref="19449052" role=""/>
                <member type="way" ref="19467829" role=""/>
                <member type="way" ref="120254982" role=""/>
                <member type="way" ref="19453659" role=""/>
                <member type="way" ref="27877068" role="forward"/>
                <member type="way" ref="27877090" role="forward"/>
                <member type="way" ref="48729802" role="forward"/>
                <member type="way" ref="48729803" role="forward"/>
                <member type="way" ref="27877092" role="forward"/>
                <member type="way" ref="27877071" role="forward"/>
                <member type="way" ref="27877073" role=""/>
                <member type="way" ref="138210981" role=""/>
                <member type="way" ref="485858399" role=""/>
                <member type="way" ref="485858398" role=""/>
                <member type="way" ref="107368817" role=""/>
                <member type="way" ref="28051355" role=""/>
                <member type="way" ref="337133131" role=""/>
                <member type="way" ref="345799667" role=""/>
                <member type="way" ref="345799666" role=""/>
                <member type="way" ref="337133130" role=""/>
                <member type="way" ref="337133126" role="forward"/>
                <member type="way" ref="337133124" role="forward"/>
                <member type="way" ref="337133128" role="forward"/>
                <member type="way" ref="337133123" role="forward"/>
                <member type="way" ref="337133129" role="forward"/>
                <member type="way" ref="337133125" role="forward"/>
                <member type="way" ref="337133127" role="forward"/>
                <member type="way" ref="337133122" role="forward"/>
                <member type="way" ref="198485817" role=""/>
                <member type="way" ref="198485816" role=""/>
                <member type="way" ref="19454855" role=""/>
                <member type="way" ref="19678814" role=""/>
                <member type="way" ref="19678827" role=""/>
                <member type="way" ref="120254878" role=""/>
                <member type="way" ref="120254879" role=""/>
                <member type="way" ref="120254877" role=""/>
                <member type="way" ref="212391155" role=""/>
                <member type="way" ref="212391151" role=""/>
                <member type="way" ref="212391164" role=""/>
                <member type="way" ref="212391128" role=""/>
                <member type="way" ref="400182722" role=""/>
                <member type="way" ref="400182733" role=""/>
                <member type="way" ref="19678482" role=""/>
                <member type="way" ref="19678820" role=""/>
                <member type="way" ref="51640799" role=""/>
                <member type="way" ref="51640802" role="forward"/>
                <member type="way" ref="51640803" role="forward"/>
                <member type="way" ref="19669531" role="forward"/>
                <member type="way" ref="120254964" role=""/>
                <member type="way" ref="392687153" role=""/>
                <member type="way" ref="392687152" role=""/>
                <member type="way" ref="19679173" role=""/>
                <member type="way" ref="19678727" role=""/>
                <member type="way" ref="111470670" role=""/>
                <member type="way" ref="19678937" role=""/>
                <member type="way" ref="132956155" role=""/>
                <member type="way" ref="132956153" role=""/>
                <member type="way" ref="111470729" role=""/>
                <member type="way" ref="120254813" role=""/>
                <member type="way" ref="120254814" role=""/>
                <member type="way" ref="19678079" role=""/>
                <member type="way" ref="120254930" role=""/>
                <member type="way" ref="120254929" role=""/>
                <member type="way" ref="343340529" role=""/>
                <member type="way" ref="120254931" role=""/>
                <member type="way" ref="27795266" role=""/>
                <member type="way" ref="27795267" role=""/>
                <member type="way" ref="19678555" role=""/>
                <member type="way" ref="19678560" role=""/>
                <member type="way" ref="111097131" role=""/>
                <member type="way" ref="51652397" role=""/>
                <member type="way" ref="120254902" role=""/>
                <member type="way" ref="120254901" role=""/>
                <member type="way" ref="111302815" role=""/>
                <member type="way" ref="111097273" role=""/>
                <member type="way" ref="279592155" role=""/>
                <member type="way" ref="19446833" role=""/>
                <member type="way" ref="19446111" role=""/>
                <member type="way" ref="19446837" role=""/>
                <member type="way" ref="19446239" role=""/>
                <member type="way" ref="19450251" role=""/>
                <member type="way" ref="19445072" role=""/>
                <member type="way" ref="19447516" role=""/>
                <member type="way" ref="120254927" role=""/>
                <member type="way" ref="120254928" role=""/>
                <member type="way" ref="200401410" role=""/>
                <member type="way" ref="200401413" role=""/>
                <member type="way" ref="120254926" role=""/>
                <member type="way" ref="271847846" role=""/>
                <member type="way" ref="271847839" role=""/>
                <member type="way" ref="271847843" role=""/>
                <member type="way" ref="110675934" role=""/>
                <member type="way" ref="271847830" role=""/>
                <member type="way" ref="271847835" role=""/>
                <member type="way" ref="19661735" role=""/>
                <member type="way" ref="271847802" role=""/>
                <member type="way" ref="120254806" role=""/>
                <member type="way" ref="120254805" role=""/>
                <member type="way" ref="169257517" role=""/>
                <member type="way" ref="169257510" role=""/>
                <member type="way" ref="169257518" role=""/>
                <member type="way" ref="169257516" role=""/>
                <member type="way" ref="19662986" role=""/>
                <member type="way" ref="19450466" role=""/>
                <member type="way" ref="19456620" role=""/>
                <member type="way" ref="359193102" role=""/>
                <member type="way" ref="120254971" role=""/>
                <member type="way" ref="120254972" role=""/>
                <member type="way" ref="19448491" role=""/>
                <member type="way" ref="51640819" role=""/>
                <member type="way" ref="19459835" role=""/>
                <member type="way" ref="111322868" role=""/>
                <member type="way" ref="469639989" role=""/>
                <member type="way" ref="473303047" role=""/>
                <member type="way" ref="473303046" role=""/>
                <member type="way" ref="19598343" role=""/>
                <member type="way" ref="19595687" role=""/>
                <member type="way" ref="19598045" role=""/>
                <member type="way" ref="19594592" role="forward"/>
                <member type="way" ref="51488544" role=""/>
                <member type="way" ref="499762839" role=""/>
                <member type="way" ref="499762840" role=""/>
                <member type="way" ref="111322900" role=""/>
                <member type="way" ref="111322855" role=""/>
                <member type="way" ref="132930955" role=""/>
                <member type="way" ref="303422154" role=""/>
                <member type="way" ref="303422156" role=""/>
                <member type="way" ref="132930956" role=""/>
                <member type="way" ref="19432672" role=""/>
                <member type="way" ref="270822274" role=""/>
                <member type="way" ref="270822273" role=""/>
                <member type="way" ref="51253243" role=""/>
                <member type="way" ref="119972399" role=""/>
                <member type="way" ref="539151319" role=""/>
                <member type="way" ref="19432102" role=""/>
                <member type="way" ref="51488545" role=""/>
                <member type="way" ref="19433162" role=""/>
                <member type="way" ref="323670634" role=""/>
                <member type="way" ref="323670628" role=""/>
                <member type="way" ref="323670631" role=""/>
                <member type="way" ref="19433142" role=""/>
                <member type="way" ref="50373959" role=""/>
                <member type="way" ref="119882185" role=""/>
                <member type="way" ref="50373957" role=""/>
                <member type="way" ref="465449754" role=""/>
                <member type="way" ref="421361425" role=""/>
                <member type="way" ref="19432209" role=""/>
                <member type="way" ref="19433164" role=""/>
                <member type="way" ref="19513781" role=""/>
                <member type="way" ref="19513768" role=""/>
                <member type="way" ref="95911112" role=""/>
                <member type="way" ref="95911113" role=""/>
                <member type="way" ref="19513770" role=""/>
                <member type="way" ref="111279908" role=""/>
                <member type="way" ref="19521773" role=""/>
                <member type="way" ref="51652683" role=""/>
                <member type="way" ref="19515483" role=""/>
                <member type="way" ref="274259087" role=""/>
                <member type="way" ref="111279914" role=""/>
                <member type="way" ref="19515737" role=""/>
                <member type="way" ref="111470705" role=""/>
                <member type="way" ref="19515718" role=""/>
                <member type="way" ref="19634248" role=""/>
                <member type="way" ref="19619429" role=""/>
                <member type="way" ref="19634241" role=""/>
                <member type="way" ref="19619419" role=""/>
                <member type="way" ref="533213614" role=""/>
                <member type="way" ref="533213613" role=""/>
                <member type="way" ref="533213612" role=""/>
                <member type="way" ref="120254787" role=""/>
                <member type="way" ref="533213617" role="forward"/>
                <member type="way" ref="533213616" role="forward"/>
                <member type="way" ref="19519041" role="forward"/>
                <member type="way" ref="173132147" role="forward"/>
                <member type="way" ref="173132148" role="forward"/>
                <member type="way" ref="120254788" role="forward"/>
                <member type="way" ref="533213626" role="forward"/>
                <member type="way" ref="533213618" role="forward"/>
                <member type="way" ref="533213621" role="forward"/>
                <member type="way" ref="51109332" role="forward"/>
                <member type="way" ref="19521367" role="forward"/>
                <member type="way" ref="280377382" role=""/>
                <member type="way" ref="111475261" role=""/>
                <member type="way" ref="116698187" role=""/>
                <member type="way" ref="116698188" role=""/>
                <member type="way" ref="52077061" role=""/>
                <member type="way" ref="49831016" role=""/>
                <member type="way" ref="49831018" role=""/>
                <member type="way" ref="458651734" role=""/>
                <member type="way" ref="458651555" role=""/>
                <member type="way" ref="49831017" role="forward"/>
                <member type="way" ref="120254936" role="forward"/>
                <member type="way" ref="49831014" role="forward"/>
                <member type="way" ref="49831015" role="forward"/>
                <member type="way" ref="49831012" role="forward"/>
                <member type="way" ref="286604680" role="forward"/>
                <member type="way" ref="49831013" role="forward"/>
                <member type="way" ref="19549634" role="forward"/>
                <member type="way" ref="286604681" role="forward"/>
                <member type="way" ref="49828458" role=""/>
                <member type="way" ref="49828457" role=""/>
                <member type="way" ref="49828456" role=""/>
                <member type="way" ref="49828459" role=""/>
                <member type="way" ref="49828699" role=""/>
                <member type="way" ref="19525794" role=""/>
                <member type="way" ref="19552678" role=""/>
                <member type="way" ref="268673426" role=""/>
                <member type="way" ref="19541037" role=""/>
                <member type="way" ref="19527201" role=""/>
                <member type="way" ref="288353571" role=""/>
                <member type="way" ref="19531284" role=""/>
                <member type="way" ref="517552735" role=""/>
                <member type="way" ref="119994721" role=""/>
                <member type="way" ref="49828221" role="forward"/>
                <member type="way" ref="51099148" role="forward"/>
                <member type="way" ref="19546232" role="forward"/>
                <member type="way" ref="49828222" role=""/>
                <member type="way" ref="203196009" role=""/>
                <member type="way" ref="56235856" role=""/>
                <member type="way" ref="56235855" role=""/>
                <member type="way" ref="225398874" role=""/>
                <member type="way" ref="225398875" role="forward"/>
                <member type="way" ref="225398876" role="forward"/>
                <member type="way" ref="278209320" role=""/>
                <member type="way" ref="278209321" role=""/>
                <member type="way" ref="270947625" role=""/>
                <member type="way" ref="120254860" role=""/>
                <member type="way" ref="120254855" role=""/>
                <member type="way" ref="271847803" role=""/>
                <member type="way" ref="271847845" role=""/>
                <member type="way" ref="134765845" role=""/>
                <member type="way" ref="134765846" role=""/>
                <member type="way" ref="19664613" role=""/>
                <member type="way" ref="89757727" role=""/>
                <member type="way" ref="309316056" role=""/>
                <member type="way" ref="255492046" role="east"/>
                <member type="way" ref="259774306" role=""/>
                <member type="way" ref="259774305" role=""/>
                <member type="way" ref="19419834" role=""/>
                <member type="way" ref="111322834" role=""/>
                <member type="way" ref="111322908" role=""/>
                <member type="way" ref="207771963" role=""/>
                <member type="way" ref="207771968" role=""/>
                <member type="way" ref="207771971" role=""/>
                <member type="way" ref="207771966" role=""/>
                <member type="way" ref="19424337" role=""/>
                <member type="way" ref="173270593" role=""/>
                <member type="way" ref="120254954" role=""/>
                <member type="way" ref="173270592" role="forward"/>
                <member type="way" ref="173270590" role="forward"/>
                <member type="way" ref="173270591" role="forward"/>
                <member type="way" ref="173270594" role="forward"/>
                <member type="way" ref="120254953" role="forward"/>
                <member type="way" ref="52300340" role="forward"/>
                <member type="way" ref="107568386" role="forward"/>
                <member type="way" ref="107568367" role="forward"/>
                <member type="way" ref="52300347" role="forward"/>
                <member type="way" ref="120254955" role=""/>
                <member type="way" ref="118011036" role=""/>
                <member type="way" ref="19547976" role=""/>
                <member type="way" ref="51640771" role=""/>
                <member type="way" ref="19505504" role=""/>
                <member type="way" ref="19506591" role=""/>
                <member type="way" ref="110888030" role=""/>
                <member type="way" ref="19505382" role=""/>
                <member type="way" ref="19508657" role=""/>
                <member type="way" ref="268683722" role=""/>
                <member type="way" ref="268683721" role=""/>
                <member type="way" ref="19506121" role=""/>
                <member type="way" ref="19644059" role=""/>
                <member type="way" ref="19644373" role=""/>
                <member type="way" ref="19644056" role=""/>
                <member type="way" ref="19644377" role=""/>
                <member type="way" ref="346987556" role=""/>
                <member type="way" ref="346987560" role=""/>
                <member type="way" ref="346987568" role=""/>
                <member type="way" ref="346987543" role=""/>
                <member type="way" ref="346987549" role=""/>
                <member type="way" ref="346987561" role=""/>
                <member type="way" ref="19644368" role=""/>
                <member type="way" ref="346987547" role=""/>
                <member type="way" ref="346987545" role=""/>
                <member type="way" ref="19461004" role=""/>
                <member type="way" ref="346987558" role=""/>
                <member type="way" ref="346987546" role=""/>
                <member type="way" ref="111491262" role=""/>
                <member type="way" ref="111491255" role=""/>
                <member type="way" ref="142096582" role=""/>
                <member type="way" ref="142096581" role="forward"/>
                <member type="way" ref="142096580" role="forward"/>
                <member type="way" ref="142096579" role="forward"/>
                <member type="way" ref="52869503" role="forward"/>
                <member type="way" ref="52869523" role="forward"/>
                <member type="way" ref="142096578" role="forward"/>
                <member type="way" ref="19654358" role=""/>
                <member type="way" ref="424817636" role=""/>
                <member type="way" ref="424817637" role=""/>
                <member type="way" ref="169502962" role=""/>
                <member type="way" ref="169502961" role=""/>
                <member type="way" ref="19617679" role=""/>
                <member type="way" ref="493725404" role=""/>
                <member type="way" ref="39235666" role=""/>
                <member type="way" ref="39235665" role=""/>
                <member type="way" ref="120254933" role=""/>
                <member type="way" ref="120254932" role=""/>
                <member type="way" ref="138211002" role=""/>
                <member type="way" ref="120254934" role=""/>
                <member type="way" ref="208938583" role=""/>
                <member type="way" ref="208938584" role=""/>
                <member type="way" ref="19652257" role=""/>
                <member type="way" ref="49502996" role="forward"/>
                <member type="way" ref="111097383" role="forward"/>
                <member type="way" ref="49503112" role="forward"/>
                <member type="way" ref="389300035" role="forward"/>
                <member type="way" ref="111097138" role="forward"/>
                <member type="way" ref="389300036" role="forward"/>
                <member type="way" ref="49503141" role="forward"/>
                <member type="way" ref="49503188" role="forward"/>
                <member type="way" ref="49503159" role="forward"/>
                <member type="way" ref="49503248" role="forward"/>
                <member type="way" ref="49502904" role="forward"/>
                <member type="way" ref="49503243" role="forward"/>
                <member type="way" ref="398901243" role=""/>
                <member type="way" ref="398903630" role=""/>
                <member type="way" ref="100281069" role=""/>
                <member type="way" ref="25887664" role=""/>
                <member type="way" ref="38722464" role=""/>
                <member type="way" ref="38722487" role=""/>
                <member type="way" ref="585508292" role=""/>
                <member type="way" ref="119725813" role=""/>
                <member type="way" ref="119725814" role=""/>
                <member type="way" ref="49500671" role=""/>
                <member type="way" ref="48942306" role=""/>
                <member type="way" ref="25887617" role=""/>
                <member type="way" ref="49500670" role=""/>
                <member type="way" ref="49500672" role=""/>
                <member type="way" ref="49500678" role="forward"/>
                <member type="way" ref="25887678" role="forward"/>
                <member type="way" ref="120254906" role="forward"/>
                <member type="way" ref="25887577" role="forward"/>
                <member type="way" ref="19644173" role="forward"/>
                <member type="way" ref="25887582" role=""/>
                <member type="way" ref="120254909" role=""/>
                <member type="way" ref="120254910" role=""/>
                <member type="way" ref="147607597" role=""/>
                <member type="way" ref="147607600" role=""/>
                <member type="way" ref="120254908" role=""/>
                <member type="way" ref="120254907" role=""/>
                <member type="way" ref="19652416" role=""/>
                <member type="way" ref="19629605" role=""/>
                <member type="way" ref="180730796" role=""/>
                <member type="way" ref="180730805" role=""/>
                <member type="way" ref="180730808" role="west"/>
                <member type="way" ref="49554802" role="west"/>
                <member type="way" ref="49554803" role="west"/>
                <member type="way" ref="208942192" role="west"/>
                <member type="way" ref="49554709" role="west"/>
                <member type="way" ref="49554708" role="west"/>
                <member type="way" ref="49554700" role="west"/>
                <member type="way" ref="49554801" role="east"/>
                <member type="way" ref="49554800" role="east"/>
                <member type="way" ref="49554811" role="east"/>
                <member type="way" ref="180730803" role="east"/>
                <member type="way" ref="49554706" role="east"/>
                <member type="way" ref="49554701" role="east"/>
                <member type="way" ref="49554831" role="east"/>
                <member type="way" ref="280377374" role=""/>
                <member type="way" ref="280388549" role=""/>
                <member type="way" ref="280377392" role=""/>
                <member type="way" ref="280377372" role=""/>
                <member type="way" ref="280377367" role=""/>
                <member type="way" ref="280377369" role=""/>
                <member type="way" ref="280377396" role=""/>
                <member type="way" ref="51640812" role="forward"/>
                <member type="way" ref="110675944" role="forward"/>
                <member type="way" ref="321393032" role=""/>
                <tag k="is_in:state" v="TN"/>
                <tag k="name" v="US 70 (TN)"/>
                <tag k="network" v="US:US"/>
                <tag k="ref" v="70"/>
                <tag k="route" v="road"/>
                <tag k="symbol" v="http://upload.wikimedia.org/wikipedia/commons/f/f8/US_70.svg"/>
                <tag k="type" v="route"/>
                <tag k="wikidata" v="Q410063"/>
                <tag k="wikipedia" v="en:U.S. Route 70 in Tennessee"/>
            </relation>
        </osm>
    "##;
    let osm: OSM = serde_xml_rs::from_str(s).unwrap();
    println!("{:#?}", osm);
}
